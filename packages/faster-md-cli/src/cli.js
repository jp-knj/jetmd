#!/usr/bin/env node

// Node CLI wrapper for faster-md
// Provides command-line interface using the faster-md JavaScript API

import { mkdir, readFile, stat, writeFile } from 'node:fs/promises'
import { basename, dirname, extname, join } from 'node:path'
import { performance } from 'node:perf_hooks'
import { fileURLToPath } from 'node:url'
import chalk from 'chalk'
import { watch } from 'chokidar'
import { program } from 'commander'
import { glob } from 'glob'
import matter from 'gray-matter'
import ora from 'ora'

// Import faster-md functions
import { VERSION, parse, renderHtml } from 'faster-md'

// Import MDX compiler if needed
import { compileMdx } from 'mdx-compiler'

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

// Setup CLI
program
  .name('faster-md')
  .description('High-performance Markdown processor')
  .version(VERSION)
  .option('-i, --input <files...>', 'input files or globs')
  .option('-o, --output <path>', 'output file or directory')
  .option('-f, --format <format>', 'output format (html, ast, mdx)', 'html')
  .option('--gfm', 'enable GitHub Flavored Markdown')
  .option('--frontmatter', 'enable frontmatter parsing')
  .option('--no-sanitize', 'disable HTML sanitization')
  .option('--mdx', 'enable MDX support')
  .option('-w, --watch', 'watch files for changes')
  .option('--quiet', 'suppress output')
  .option('--verbose', 'verbose output')
  .option('--time', 'show timing information')
  .option('--config <path>', 'path to config file')

// Parse command
program
  .command('parse [files...]')
  .description('Parse markdown to AST')
  .option('--pretty', 'pretty print JSON')
  .option('--positions', 'include position information')
  .action(async (files, options) => {
    await handleParse(files, { ...program.opts(), ...options })
  })

// Render command
program
  .command('render [files...]')
  .description('Render markdown to HTML')
  .option('--document', 'wrap in HTML document')
  .option('--title <title>', 'document title', 'Document')
  .option('--css <path>', 'CSS file to include')
  .action(async (files, options) => {
    await handleRender(files, { ...program.opts(), ...options })
  })

// Compile command (for MDX)
program
  .command('compile [files...]')
  .description('Compile MDX to JavaScript')
  .option('--jsx-runtime <runtime>', 'JSX runtime (classic or automatic)', 'automatic')
  .option('--jsx-import-source <source>', 'JSX import source', 'react')
  .action(async (files, options) => {
    await handleCompile(files, { ...program.opts(), ...options })
  })

// Check command
program
  .command('check [files...]')
  .description('Validate markdown files')
  .option('--strict', 'strict CommonMark mode')
  .action(async (files, options) => {
    await handleCheck(files, { ...program.opts(), ...options })
  })

// Stats command
program
  .command('stats [files...]')
  .description('Show statistics about markdown files')
  .option('--detailed', 'show detailed statistics')
  .action(async (files, options) => {
    await handleStats(files, { ...program.opts(), ...options })
  })

// Bench command
program
  .command('bench [files...]')
  .description('Benchmark markdown processing')
  .option('-n, --iterations <n>', 'number of iterations', '100')
  .option('--warmup <n>', 'warmup iterations', '10')
  .action(async (files, options) => {
    await handleBench(files, { ...program.opts(), ...options })
  })

// Parse arguments
program.parse(process.argv)

// Default action
if (!process.argv.slice(2).length) {
  program.outputHelp()
}

// Main handler for default processing
async function processFiles(files, options) {
  const spinner = options.quiet ? null : ora('Processing files...').start()

  try {
    const inputFiles = await resolveFiles(files || options.input || [])

    if (inputFiles.length === 0) {
      if (!options.quiet) {
        console.warn(chalk.yellow('No input files specified'))
      }
      process.exit(1)
    }

    const results = []
    const startTime = performance.now()

    for (const file of inputFiles) {
      try {
        const result = await processFile(file, options)
        results.push(result)

        if (!options.quiet && spinner) {
          spinner.text = `Processed ${file}`
        }
      } catch (error) {
        if (!options.quiet) {
          console.error(chalk.red(`Error processing ${file}:`), error.message)
        }
        if (!options.watch) {
          process.exit(1)
        }
      }
    }

    const endTime = performance.now()

    if (spinner) {
      spinner.succeed(`Processed ${results.length} files`)
    }

    if (options.time && !options.quiet) {
      const elapsed = endTime - startTime
      console.log(chalk.gray(`Time: ${elapsed.toFixed(2)}ms`))

      const totalBytes = results.reduce((sum, r) => sum + r.inputSize, 0)
      const throughput = totalBytes / 1024 / 1024 / (elapsed / 1000)
      console.log(chalk.gray(`Throughput: ${throughput.toFixed(2)} MB/s`))
    }

    // Set up file watching if requested
    if (options.watch) {
      setupWatcher(inputFiles, options)
    }
  } catch (error) {
    if (spinner) {
      spinner.fail('Processing failed')
    }
    console.error(chalk.red('Error:'), error.message)
    process.exit(1)
  }
}

// Process a single file
async function processFile(inputPath, options) {
  const content = await readFile(inputPath, 'utf-8')
  const inputSize = Buffer.byteLength(content, 'utf-8')

  let output
  let outputPath

  // Handle frontmatter if needed
  let processedContent = content
  let frontmatterData = {}

  if (options.frontmatter) {
    const parsed = matter(content)
    processedContent = parsed.content
    frontmatterData = parsed.data
  }

  // Process based on format
  switch (options.format) {
    case 'ast': {
      const ast = await parse(processedContent, {
        gfm: options.gfm,
        mdx: options.mdx,
        position: true,
      })
      output = JSON.stringify(ast, null, options.pretty ? 2 : 0)
      break
    }

    case 'mdx': {
      const mdxResult = await compileMdx(processedContent, {
        jsxRuntime: options.jsxRuntime || 'automatic',
        jsxImportSource: options.jsxImportSource || 'react',
        development: process.env.NODE_ENV === 'development',
      })
      output = mdxResult.code
      break
    }
    default:
      output = await renderHtml(processedContent, {
        gfm: options.gfm,
        sanitize: options.sanitize !== false,
        xhtml: false,
      })

      // Wrap in document if requested
      if (options.document) {
        output = wrapInDocument(output, {
          title: frontmatterData.title || options.title || 'Document',
          css: options.css,
        })
      }
      break
  }

  // Determine output path
  if (options.output) {
    const outputStat = await stat(options.output).catch(() => null)

    if (outputStat?.isDirectory()) {
      // Output to directory with same name but different extension
      const ext = options.format === 'mdx' ? '.js' : options.format === 'ast' ? '.json' : '.html'
      const name = basename(inputPath, extname(inputPath))
      outputPath = join(options.output, name + ext)
    } else {
      outputPath = options.output
    }

    // Create output directory if needed
    await mkdir(dirname(outputPath), { recursive: true })
    await writeFile(outputPath, output, 'utf-8')

    if (options.verbose && !options.quiet) {
      console.log(chalk.green('✓'), `${inputPath} → ${outputPath}`)
    }
  } else {
    // Output to stdout
    if (!options.quiet) {
      console.log(output)
    }
  }

  return {
    inputPath,
    outputPath,
    inputSize,
    outputSize: Buffer.byteLength(output, 'utf-8'),
    frontmatter: frontmatterData,
  }
}

// Handler functions for commands
async function handleParse(files, options) {
  const inputFiles = await resolveFiles(files || options.input || [])

  for (const file of inputFiles) {
    const content = await readFile(file, 'utf-8')
    const ast = await parse(content, {
      gfm: options.gfm,
      frontmatter: options.frontmatter,
      mdx: options.mdx,
      position: options.positions,
    })

    const json = JSON.stringify(ast, null, options.pretty ? 2 : 0)

    if (options.output) {
      await writeFile(options.output, json, 'utf-8')
    } else {
      console.log(json)
    }
  }
}

async function handleRender(files, options) {
  options.format = 'html'
  await processFiles(files, options)
}

async function handleCompile(files, options) {
  options.format = 'mdx'
  await processFiles(files, options)
}

async function handleCheck(files, options) {
  const inputFiles = await resolveFiles(files || options.input || [])
  let hasErrors = false

  for (const file of inputFiles) {
    try {
      const content = await readFile(file, 'utf-8')
      await parse(content, {
        gfm: !options.strict,
        frontmatter: options.frontmatter,
        mdx: options.mdx,
      })

      if (!options.quiet) {
        console.log(chalk.green('✓'), file)
      }
    } catch (error) {
      hasErrors = true
      console.error(chalk.red('✗'), file, '-', error.message)
    }
  }

  if (hasErrors) {
    process.exit(1)
  }
}

async function handleStats(files, options) {
  const inputFiles = await resolveFiles(files || options.input || [])

  for (const file of inputFiles) {
    const content = await readFile(file, 'utf-8')
    const lines = content.split('\n').length
    const words = content.split(/\s+/).filter(Boolean).length
    const chars = content.length

    console.log(chalk.bold(file))
    console.log(`  Lines: ${lines}`)
    console.log(`  Words: ${words}`)
    console.log(`  Characters: ${chars}`)

    if (options.detailed) {
      const ast = await parse(content, { gfm: options.gfm })
      const stats = collectStats(ast)

      console.log(`  Paragraphs: ${stats.paragraphs}`)
      console.log(`  Headings: ${stats.headings}`)
      console.log(`  Links: ${stats.links}`)
      console.log(`  Images: ${stats.images}`)
      console.log(`  Code blocks: ${stats.codeBlocks}`)
    }

    console.log()
  }
}

async function handleBench(files, options) {
  const inputFiles = await resolveFiles(files || options.input || [])
  const iterations = Number.parseInt(options.iterations, 10)
  const warmup = Number.parseInt(options.warmup, 10)

  for (const file of inputFiles) {
    const content = await readFile(file, 'utf-8')
    const size = Buffer.byteLength(content, 'utf-8')

    console.log(chalk.bold(file))
    console.log(`  File size: ${size} bytes`)

    // Warmup
    for (let i = 0; i < warmup; i++) {
      await renderHtml(content, { gfm: options.gfm })
    }

    // Benchmark
    const times = []
    for (let i = 0; i < iterations; i++) {
      const start = performance.now()
      await renderHtml(content, { gfm: options.gfm })
      const end = performance.now()
      times.push(end - start)
    }

    // Calculate statistics
    times.sort((a, b) => a - b)
    const mean = times.reduce((a, b) => a + b, 0) / times.length
    const median = times[Math.floor(times.length / 2)]
    const p95 = times[Math.floor(times.length * 0.95)]
    const p99 = times[Math.floor(times.length * 0.99)]
    const throughput = size / 1024 / 1024 / (mean / 1000)

    console.log(`  Mean: ${mean.toFixed(2)}ms`)
    console.log(`  Median: ${median.toFixed(2)}ms`)
    console.log(`  P95: ${p95.toFixed(2)}ms`)
    console.log(`  P99: ${p99.toFixed(2)}ms`)
    console.log(`  Throughput: ${throughput.toFixed(2)} MB/s`)
    console.log()
  }
}

// Utility functions
async function resolveFiles(patterns) {
  if (patterns.length === 0) {
    return []
  }

  const files = []
  for (const pattern of patterns) {
    const matches = await glob(pattern, {
      ignore: ['**/node_modules/**', '**/.git/**'],
    })
    files.push(...matches)
  }

  return [...new Set(files)] // Remove duplicates
}

function setupWatcher(files, options) {
  console.log(chalk.blue('Watching for changes...'))

  const watcher = watch(files, {
    persistent: true,
    ignoreInitial: true,
  })

  watcher.on('change', async (path) => {
    console.log(chalk.yellow(`File changed: ${path}`))
    try {
      await processFile(path, options)
    } catch (error) {
      console.error(chalk.red(`Error processing ${path}:`), error.message)
    }
  })

  watcher.on('error', (error) => {
    console.error(chalk.red('Watcher error:'), error)
  })

  // Handle graceful shutdown
  process.on('SIGINT', () => {
    watcher.close()
    console.log(chalk.blue('\nStopped watching'))
    process.exit(0)
  })
}

function wrapInDocument(html, options = {}) {
  const { title = 'Document', css } = options

  let cssContent = ''
  if (css) {
    try {
      const cssPath = join(process.cwd(), css)
      cssContent = `<style>${readFileSync(cssPath, 'utf-8')}</style>`
    } catch (_error) {
      console.warn(chalk.yellow(`Warning: Could not read CSS file: ${css}`))
    }
  }

  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${title}</title>
    ${cssContent}
    <style>
        body {
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            line-height: 1.6;
        }
        pre {
            background: #f6f8fa;
            padding: 1rem;
            border-radius: 6px;
            overflow-x: auto;
        }
        code {
            background: #f6f8fa;
            padding: 0.2em 0.4em;
            border-radius: 3px;
            font-size: 0.9em;
        }
    </style>
</head>
<body>
    <main>
        ${html}
    </main>
</body>
</html>`
}

function collectStats(ast) {
  const stats = {
    paragraphs: 0,
    headings: 0,
    links: 0,
    images: 0,
    codeBlocks: 0,
  }

  function walk(node) {
    if (!node) return

    switch (node.type) {
      case 'paragraph':
        stats.paragraphs++
        break
      case 'heading':
        stats.headings++
        break
      case 'link':
        stats.links++
        break
      case 'image':
        stats.images++
        break
      case 'code':
        stats.codeBlocks++
        break
    }

    if (node.children) {
      for (const child of node.children) {
        walk(child)
      }
    }
  }

  walk(ast)
  return stats
}

// Export for programmatic use
export { processFile, processFiles, resolveFiles }
