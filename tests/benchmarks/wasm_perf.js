#!/usr/bin/env node

/**
 * WASM Performance Benchmark
 * Target: ≥50 MB/s throughput
 */

import { fileURLToPath } from 'node:url'
import { readFile } from 'node:fs/promises'
import { join, dirname } from 'node:path'
import { performance } from 'node:perf_hooks'
import { renderHtml, parse } from '../../packages/faster-md/dist/index.js'

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

// Test documents of various sizes
const TEST_DOCS = {
  small: {
    name: 'Small (1KB)',
    size: 1024,
    content: generateMarkdown(1024),
  },
  medium: {
    name: 'Medium (50KB)',
    size: 50 * 1024,
    content: generateMarkdown(50 * 1024),
  },
  large: {
    name: 'Large (500KB)',
    size: 500 * 1024,
    content: generateMarkdown(500 * 1024),
  },
  xlarge: {
    name: 'Extra Large (5MB)',
    size: 5 * 1024 * 1024,
    content: generateMarkdown(5 * 1024 * 1024),
  },
}

/**
 * Generate markdown content of specified size
 */
function generateMarkdown(targetSize) {
  const sections = []
  let currentSize = 0

  // Common markdown patterns
  const patterns = [
    '# Heading Level 1\n\nThis is a paragraph with some **bold text** and *italic text*.\n\n',
    '## Heading Level 2\n\nAnother paragraph with a [link](https://example.com) and `inline code`.\n\n',
    '### Heading Level 3\n\n- List item 1\n- List item 2\n- List item 3\n\n',
    '```javascript\nfunction example() {\n  return "Hello, World!";\n}\n```\n\n',
    '| Column 1 | Column 2 | Column 3 |\n|----------|----------|----------|\n| Cell 1   | Cell 2   | Cell 3   |\n\n',
    '> This is a blockquote with some text inside it.\n> It can span multiple lines.\n\n',
    '![Alt text](image.jpg "Image title")\n\n',
    '---\n\n',
  ]

  while (currentSize < targetSize) {
    const pattern = patterns[sections.length % patterns.length]
    sections.push(pattern)
    currentSize += Buffer.byteLength(pattern, 'utf-8')
  }

  return sections.join('')
}

/**
 * Run benchmark for a specific document
 */
async function runBenchmark(doc, iterations = 100) {
  console.log(`\nBenchmarking: ${doc.name}`)
  console.log(`Document size: ${(doc.size / 1024).toFixed(1)} KB`)
  console.log(`Iterations: ${iterations}`)

  // Warmup
  for (let i = 0; i < 10; i++) {
    await renderHtml(doc.content, { gfm: true })
  }

  // Benchmark
  const times = []
  const start = performance.now()

  for (let i = 0; i < iterations; i++) {
    const iterStart = performance.now()
    await renderHtml(doc.content, { gfm: true })
    const iterEnd = performance.now()
    times.push(iterEnd - iterStart)
  }

  const end = performance.now()
  const totalTime = end - start

  // Calculate statistics
  times.sort((a, b) => a - b)
  const mean = times.reduce((a, b) => a + b, 0) / times.length
  const median = times[Math.floor(times.length / 2)]
  const p50 = median
  const p95 = times[Math.floor(times.length * 0.95)]
  const p99 = times[Math.floor(times.length * 0.99)]

  // Calculate throughput
  const bytesPerSecond = (doc.size * iterations) / (totalTime / 1000)
  const mbPerSecond = bytesPerSecond / (1024 * 1024)

  // Display results
  console.log('\nResults:')
  console.log(`  Mean time: ${mean.toFixed(2)} ms`)
  console.log(`  Median (P50): ${p50.toFixed(2)} ms`)
  console.log(`  P95: ${p95.toFixed(2)} ms`)
  console.log(`  P99: ${p99.toFixed(2)} ms`)
  console.log(`  Total time: ${(totalTime / 1000).toFixed(2)} s`)
  console.log(`  Throughput: ${mbPerSecond.toFixed(2)} MB/s`)

  return {
    name: doc.name,
    size: doc.size,
    mean,
    median,
    p50,
    p95,
    p99,
    throughput: mbPerSecond,
  }
}

/**
 * Run parse performance benchmark
 */
async function runParseBenchmark(doc, iterations = 100) {
  console.log(`\nParse Benchmark: ${doc.name}`)

  // Warmup
  for (let i = 0; i < 10; i++) {
    await parse(doc.content, { gfm: true })
  }

  const times = []
  const start = performance.now()

  for (let i = 0; i < iterations; i++) {
    const iterStart = performance.now()
    await parse(doc.content, { gfm: true })
    const iterEnd = performance.now()
    times.push(iterEnd - iterStart)
  }

  const end = performance.now()
  const totalTime = end - start

  times.sort((a, b) => a - b)
  const mean = times.reduce((a, b) => a + b, 0) / times.length
  const median = times[Math.floor(times.length / 2)]

  const bytesPerSecond = (doc.size * iterations) / (totalTime / 1000)
  const mbPerSecond = bytesPerSecond / (1024 * 1024)

  console.log(`  Parse throughput: ${mbPerSecond.toFixed(2)} MB/s`)
  console.log(`  Parse mean: ${mean.toFixed(2)} ms`)
  console.log(`  Parse median: ${median.toFixed(2)} ms`)

  return mbPerSecond
}

/**
 * Main benchmark runner
 */
async function main() {
  console.log('=' * 60)
  console.log('WASM Performance Benchmark')
  console.log('=' * 60)

  const results = []
  const TARGET_THROUGHPUT = 50 // MB/s

  // Run benchmarks for each document size
  for (const [key, doc] of Object.entries(TEST_DOCS)) {
    // Skip extra large for quick tests
    if (key === 'xlarge' && process.env.QUICK_TEST) {
      continue
    }

    const iterations = key === 'xlarge' ? 10 : 100
    const result = await runBenchmark(doc, iterations)
    results.push(result)

    // Also run parse benchmark
    await runParseBenchmark(doc, iterations)
  }

  // Summary
  console.log('\n' + '=' * 60)
  console.log('SUMMARY')
  console.log('=' * 60)

  console.log('\n Performance by Document Size:')
  for (const result of results) {
    const status = result.throughput >= TARGET_THROUGHPUT ? '✅ PASS' : '❌ FAIL'
    console.log(
      `  ${result.name}: ${result.throughput.toFixed(2)} MB/s (${
        result.median.toFixed(2)
      } ms median) ${status}`,
    )
  }

  // Overall assessment
  const avgThroughput = results.reduce((a, b) => a + b.throughput, 0) / results.length
  const passedTarget = avgThroughput >= TARGET_THROUGHPUT

  console.log('\n' + '-' * 60)
  console.log(`Average throughput: ${avgThroughput.toFixed(2)} MB/s`)
  console.log(`Target throughput: ${TARGET_THROUGHPUT} MB/s`)
  console.log(
    `Status: ${passedTarget ? '✅ PASS' : '❌ FAIL'} - ${
      passedTarget ? 'Met performance target!' : 'Below performance target'
    }`,
  )

  // Check 50KB document specifically (T073 requirement)
  const mediumResult = results.find((r) => r.name === 'Medium (50KB)')
  if (mediumResult) {
    console.log('\n50KB Document Performance (T073):')
    console.log(`  P50 latency: ${mediumResult.p50.toFixed(2)} ms`)
    console.log(`  Target: <3ms`)
    console.log(`  Status: ${mediumResult.p50 < 3 ? '✅ PASS' : '❌ FAIL'}`)
  }

  process.exit(passedTarget ? 0 : 1)
}

// Run if executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch((error) => {
    console.error('Benchmark failed:', error)
    process.exit(1)
  })
}

export { runBenchmark, runParseBenchmark, generateMarkdown }