// MDX compiler - Compiles MDX to ES modules
export const VERSION = '0.1.0'

import { parse } from 'faster-md'
import { createCodeGenerator } from './codegen.js'
import { createJsxParser } from './jsx.js'
import { createTokenizer } from './tokenizer.js'

/**
 * MDX compilation options
 * @typedef {Object} CompileOptions
 * @property {string} [filename] - Source filename for debugging
 * @property {boolean} [development=false] - Development mode
 * @property {string} [jsxRuntime='classic'] - JSX runtime (classic or automatic)
 * @property {string} [jsxImportSource='react'] - JSX import source
 * @property {string} [pragma='React.createElement'] - JSX pragma
 * @property {string} [pragmaFrag='React.Fragment'] - Fragment pragma
 * @property {boolean} [outputFormat='program'] - Output format (program or function-body)
 * @property {Array} [remarkPlugins=[]] - Remark plugins to use
 * @property {Array} [rehypePlugins=[]] - Rehype plugins to use
 * @property {Object} [mdxOptions] - Additional MDX-specific options
 */

/**
 * Compilation result
 * @typedef {Object} CompileResult
 * @property {string} code - Generated ES module code
 * @property {Object} [map] - Source map (if requested)
 * @property {Object} [data] - Additional compilation data
 * @property {Array} [messages] - Compilation messages/warnings
 */

/**
 * MDX compiler
 */
export class MDXCompiler {
  constructor(options = {}) {
    this.options = {
      development: false,
      jsxRuntime: 'classic',
      jsxImportSource: 'react',
      pragma: 'React.createElement',
      pragmaFrag: 'React.Fragment',
      outputFormat: 'program',
      remarkPlugins: [],
      rehypePlugins: [],
      ...options,
    }

    this.tokenizer = createTokenizer()
    this.jsxParser = createJsxParser()
    this.codeGenerator = createCodeGenerator({
      pragma: this.options.pragma,
      pragmaFrag: this.options.pragmaFrag,
      jsxRuntime: this.options.jsxRuntime,
      jsxImportSource: this.options.jsxImportSource,
      development: this.options.development,
    })
  }

  /**
   * Compile MDX to ES module
   * @param {string} mdx - MDX content
   * @returns {Promise<CompileResult>} Compilation result
   */
  async compile(mdx) {
    const messages = []

    try {
      // Step 1: Tokenize MDX content
      const tokens = this.tokenizer.tokenize(mdx)

      // Step 2: Process markdown tokens through remark
      const processedTokens = await this.processMarkdownTokens(tokens)

      // Step 3: Validate JSX tokens
      const validatedTokens = this.validateJsxTokens(processedTokens, messages)

      // Step 4: Generate ES module code
      const code = this.codeGenerator.generate(validatedTokens, {
        filename: this.options.filename,
        componentName: this.options.componentName,
        frontmatter: this.options.frontmatter !== false,
      })

      // Step 5: Format based on output format
      const formattedCode = this.formatOutput(code)

      return {
        code: formattedCode,
        data: {
          tokens: validatedTokens,
          imports: Array.from(this.codeGenerator.imports),
          exports: Array.from(this.codeGenerator.exports.keys()),
          components: Array.from(this.codeGenerator.components),
        },
        messages,
      }
    } catch (error) {
      throw new Error(`MDX compilation failed: ${error.message}`)
    }
  }

  /**
   * Compile synchronously
   * @param {string} mdx - MDX content
   * @returns {CompileResult} Compilation result
   */
  compileSync(mdx) {
    const messages = []

    try {
      // Tokenize
      const tokens = this.tokenizer.tokenize(mdx)

      // Process markdown tokens (sync version)
      const processedTokens = this.processMarkdownTokensSync(tokens)

      // Validate JSX
      const validatedTokens = this.validateJsxTokens(processedTokens, messages)

      // Generate code
      const code = this.codeGenerator.generate(validatedTokens, {
        filename: this.options.filename,
        componentName: this.options.componentName,
        frontmatter: this.options.frontmatter !== false,
      })

      // Format output
      const formattedCode = this.formatOutput(code)

      return {
        code: formattedCode,
        data: {
          tokens: validatedTokens,
          imports: Array.from(this.codeGenerator.imports),
          exports: Array.from(this.codeGenerator.exports.keys()),
          components: Array.from(this.codeGenerator.components),
        },
        messages,
      }
    } catch (error) {
      throw new Error(`MDX compilation failed: ${error.message}`)
    }
  }

  /**
   * Process markdown tokens through remark plugins
   * @private
   */
  async processMarkdownTokens(tokens) {
    if (this.options.remarkPlugins.length === 0) {
      return tokens
    }

    // Process markdown tokens through remark
    // This is simplified - full implementation would properly
    // integrate with remark pipeline
    let processedTokens = tokens

    for (const plugin of this.options.remarkPlugins) {
      if (typeof plugin === 'function') {
        processedTokens = await plugin(processedTokens, this)
      }
    }

    return processedTokens
  }

  /**
   * Process markdown tokens synchronously
   * @private
   */
  processMarkdownTokensSync(tokens) {
    if (this.options.remarkPlugins.length === 0) {
      return tokens
    }

    let processedTokens = tokens

    for (const plugin of this.options.remarkPlugins) {
      if (typeof plugin === 'function') {
        processedTokens = plugin(processedTokens, this)
      }
    }

    return processedTokens
  }

  /**
   * Validate JSX tokens
   * @private
   */
  validateJsxTokens(tokens, messages) {
    const validated = []

    for (const token of tokens) {
      if (token.type === 'jsx_element' || token.type === 'jsx_expression') {
        const validation = this.jsxParser.validate(token.value)

        if (!validation.valid) {
          messages.push({
            type: 'warning',
            message: `Invalid JSX: ${validation.error}`,
            location: validation.location || { start: token.start, end: token.end },
          })

          // Skip invalid JSX in strict mode
          if (this.options.strict) {
            continue
          }
        }
      }

      validated.push(token)
    }

    return validated
  }

  /**
   * Format output based on output format option
   * @private
   */
  formatOutput(code) {
    if (this.options.outputFormat === 'function-body') {
      // Return just the component function body
      const match = code.match(/function\s+\w+\s*\([^)]*\)\s*{([\s\S]*?)^}/m)
      if (match) {
        return match[1].trim()
      }
    }

    return code
  }

  /**
   * Create a processor with plugins
   * @param {Array} plugins - Plugins to use
   * @returns {MDXCompiler} Configured compiler
   */
  use(plugin, options) {
    if (typeof plugin === 'function') {
      this.options.remarkPlugins.push(plugin(options))
    } else {
      this.options.remarkPlugins.push(plugin)
    }

    return this
  }
}

/**
 * Compile MDX to ES module
 * @param {string} mdx - MDX content
 * @param {CompileOptions} [options] - Compilation options
 * @returns {Promise<CompileResult>} Compilation result
 */
export async function compileMdx(mdx, options = {}) {
  const compiler = new MDXCompiler(options)
  return compiler.compile(mdx)
}

/**
 * Compile MDX synchronously
 * @param {string} mdx - MDX content
 * @param {CompileOptions} [options] - Compilation options
 * @returns {CompileResult} Compilation result
 */
export function compileMdxSync(mdx, options = {}) {
  const compiler = new MDXCompiler(options)
  return compiler.compileSync(mdx)
}

/**
 * Create an MDX compiler instance
 * @param {CompileOptions} [options] - Compilation options
 * @returns {MDXCompiler} Compiler instance
 */
export function createCompiler(options) {
  return new MDXCompiler(options)
}

/**
 * Evaluate compiled MDX code
 * @param {string} code - Compiled ES module code
 * @param {Object} [scope] - Variables to make available
 * @returns {Object} Module exports
 */
export async function evaluateMdx(code, scope = {}) {
  // This is a simplified implementation
  // Full implementation would use a proper module evaluation strategy

  const AsyncFunction = Object.getPrototypeOf(async () => {}).constructor
  const keys = Object.keys(scope)
  const values = Object.values(scope)

  try {
    const fn = new AsyncFunction(...keys, 'exports', `${code}\nreturn exports`)
    const exports = {}
    await fn(...values, exports)
    return exports
  } catch (error) {
    throw new Error(`MDX evaluation failed: ${error.message}`)
  }
}

/**
 * Quick compile with default options
 * @param {string} mdx - MDX content
 * @returns {Promise<string>} Compiled code
 */
export async function quickCompile(mdx) {
  const result = await compileMdx(mdx, {
    jsxRuntime: 'automatic',
    development: process.env.NODE_ENV === 'development',
  })

  return result.code
}
