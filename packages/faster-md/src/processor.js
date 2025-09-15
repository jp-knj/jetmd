// Processor class for faster-md
// Provides a unified, chainable API for markdown processing with plugin support

import { createParseSession, parse, parseIncremental, parseSync } from './parse.js'
import { renderHtml, renderHtmlSync } from './render.js'

/**
 * Plugin interface
 * @typedef {Object} Plugin
 * @property {string} name - Plugin name
 * @property {Function} [parser] - Parser transformer
 * @property {Function} [compiler] - Compiler transformer
 * @property {Function} [transformer] - AST transformer
 * @property {Object} [options] - Plugin options
 */

/**
 * Processor options
 * @typedef {Object} ProcessorOptions
 * @property {boolean} [gfm=false] - Enable GitHub Flavored Markdown
 * @property {boolean} [frontmatter=false] - Enable frontmatter parsing
 * @property {boolean} [sanitize=true] - Enable HTML sanitization
 * @property {boolean} [mdx=false] - Enable MDX support
 * @property {Object} [data] - Custom data to attach to processor
 */

/**
 * Markdown processor with plugin support
 */
export class Processor {
  constructor(options = {}) {
    this.options = {
      gfm: false,
      frontmatter: false,
      sanitize: true,
      mdx: false,
      ...options,
    }

    this.plugins = []
    this.data = options.data || {}
    this.sessionId = null
    this.frozen = false
  }

  /**
   * Use a plugin
   * @param {Plugin|Function} plugin - Plugin to use
   * @param {Object} [options] - Plugin options
   * @returns {Processor} This processor for chaining
   */
  use(plugin, options) {
    if (this.frozen) {
      throw new Error('Cannot add plugins to a frozen processor')
    }

    // Handle function plugins
    let processedPlugin = plugin
    if (typeof plugin === 'function') {
      processedPlugin = plugin(options)
    }

    // Validate plugin
    if (!processedPlugin || typeof processedPlugin !== 'object') {
      throw new TypeError('Plugin must be an object or function that returns an object')
    }

    if (!processedPlugin.name) {
      throw new Error('Plugin must have a name')
    }

    // Store plugin with options
    this.plugins.push({
      ...processedPlugin,
      options: { ...processedPlugin.options, ...options },
    })

    return this
  }

  /**
   * Set processor data
   * @param {string} key - Data key
   * @param {*} value - Data value
   * @returns {Processor} This processor for chaining
   */
  data(...args) {
    if (args.length === 0) {
      return this.data
    }

    if (args.length === 1) {
      return this.data[args[0]]
    }

    this.data[args[0]] = args[1]
    return this
  }

  /**
   * Freeze the processor (prevent further modifications)
   * @returns {Processor} This processor for chaining
   */
  freeze() {
    this.frozen = true
    return this
  }

  /**
   * Process markdown to AST
   * @param {string} markdown - Markdown content
   * @returns {Promise<Object>} Parsed AST
   */
  async parse(markdown) {
    // Apply parser plugins
    let processedMarkdown = markdown
    for (const plugin of this.plugins) {
      if (plugin.parser) {
        processedMarkdown = await plugin.parser(processedMarkdown, this)
      }
    }

    // Parse with options
    let ast = await parse(processedMarkdown, this.options)

    // Apply transformer plugins
    for (const plugin of this.plugins) {
      if (plugin.transformer) {
        ast = await plugin.transformer(ast, this)
      }
    }

    return ast
  }

  /**
   * Process markdown to HTML
   * @param {string} markdown - Markdown content
   * @returns {Promise<string>} Rendered HTML
   */
  async process(markdown) {
    // Parse to AST
    const ast = await this.parse(markdown)

    // Compile to HTML
    return this.compile(ast)
  }

  /**
   * Compile AST to HTML
   * @param {Object} ast - Parsed AST
   * @returns {Promise<string>} Rendered HTML
   */
  async compile(ast) {
    // Apply compiler plugins
    let processedAst = ast
    for (const plugin of this.plugins) {
      if (plugin.compiler) {
        processedAst = await plugin.compiler(processedAst, this)
      }
    }

    // Render to HTML
    return renderHtml(processedAst, this.options)
  }

  /**
   * Process markdown synchronously
   * @param {string} markdown - Markdown content
   * @returns {string} Rendered HTML
   */
  processSync(markdown) {
    // Apply parser plugins synchronously
    let processedMarkdown = markdown
    for (const plugin of this.plugins) {
      if (plugin.parser) {
        // Assume sync version exists
        processedMarkdown = plugin.parser(processedMarkdown, this)
      }
    }

    // Parse synchronously
    let ast = parseSync(processedMarkdown, this.options)

    // Apply transformer plugins synchronously
    for (const plugin of this.plugins) {
      if (plugin.transformer) {
        ast = plugin.transformer(ast, this)
      }
    }

    // Apply compiler plugins synchronously
    for (const plugin of this.plugins) {
      if (plugin.compiler) {
        ast = plugin.compiler(ast, this)
      }
    }

    // Render synchronously
    return renderHtmlSync(ast, this.options)
  }

  /**
   * Enable incremental processing
   * @param {string} [sessionId] - Optional session ID
   * @returns {Promise<Processor>} This processor for chaining
   */
  async enableIncremental(sessionId) {
    this.sessionId = await createParseSession(sessionId)
    return this
  }

  /**
   * Process incrementally with edits
   * @param {string} markdown - Updated markdown
   * @param {Array} edits - Edit operations
   * @returns {Promise<string>} Rendered HTML
   */
  async processIncremental(markdown, edits) {
    if (!this.sessionId) {
      throw new Error('Incremental processing not enabled. Call enableIncremental() first.')
    }

    const ast = await parseIncremental(this.sessionId, markdown, edits)
    return this.compile(ast)
  }

  /**
   * Create a copy of this processor
   * @returns {Processor} New processor with same configuration
   */
  copy() {
    const newProcessor = new Processor(this.options)
    newProcessor.plugins = [...this.plugins]
    newProcessor.data = { ...this.data }
    return newProcessor
  }

  /**
   * Run the processor in a specific mode
   * @param {string} mode - Mode to run in ('parse', 'compile', 'process')
   * @returns {Function} Runner function
   */
  run(mode = 'process') {
    const processor = this.freeze()

    switch (mode) {
      case 'parse':
        return (markdown) => processor.parse(markdown)
      case 'compile':
        return (ast) => processor.compile(ast)
      default:
        return (markdown) => processor.process(markdown)
    }
  }
}

/**
 * Create a new processor
 * @param {ProcessorOptions} [options] - Processor options
 * @returns {Processor} New processor instance
 */
export function createProcessor(options) {
  return new Processor(options)
}

/**
 * Quick process with default options
 * @param {string} markdown - Markdown content
 * @param {ProcessorOptions} [options] - Options
 * @returns {Promise<string>} Rendered HTML
 */
export async function quickProcess(markdown, options = {}) {
  const processor = createProcessor({
    gfm: true,
    sanitize: true,
    ...options,
  })

  return processor.process(markdown)
}
