// Render wrapper for faster-md
// Wraps the WASM renderHtml function with JavaScript-friendly interface

// Cache functions available but not currently used in this module
import { getWasmInstance } from './loader.js'
import { parse } from './parse.js'

/**
 * HTML render options
 * @typedef {Object} RenderOptions
 * @property {boolean} [sanitize=true] - Enable HTML sanitization
 * @property {boolean} [allowDangerousHtml=false] - Allow raw HTML (bypasses sanitization)
 * @property {boolean} [gfm=false] - Enable GitHub Flavored Markdown
 * @property {boolean} [xhtml=false] - Use XHTML-style self-closing tags
 * @property {Object} [sanitizeOptions] - Custom sanitization options
 * @property {string} [sessionId] - Session ID for incremental rendering
 */

/**
 * Render Markdown to HTML
 * @param {string|object} input - Markdown string or parsed AST
 * @param {RenderOptions} [options={}] - Rendering options
 * @returns {Promise<string>} The rendered HTML
 */
export async function renderHtml(input, options = {}) {
  // Check cache for string inputs
  if (typeof input === 'string') {
    const cached = getCached(input, options)
    if (cached) {
      return cached
    }
  }

  const wasm = await getWasmInstance()

  try {
    // If input is a string, use direct renderHtml
    if (typeof input === 'string') {
      // Pass options object directly, not JSON string
      const html = wasm.renderHtml(input, options)
      // Cache the result
      setCached(input, options, html)
      return html
    }

    // If input is an AST object, use renderAstToHtml
    if (typeof input === 'object' && input !== null) {
      if (wasm.renderAstToHtml) {
        const html = wasm.renderAstToHtml(JSON.stringify(input), JSON.stringify(options))
        return html
      }
      // Fallback: convert AST back to markdown and render
      // This is not ideal but provides compatibility
      throw new Error('AST rendering not yet implemented in WASM module')
    }

    throw new TypeError('Input must be a string (markdown) or object (AST)')
  } catch (error) {
    const enhancedError = new Error(`Render error: ${error.message}`)
    enhancedError.originalError = error
    throw enhancedError
  }
}

/**
 * Render Markdown to HTML synchronously (requires WASM to be pre-initialized)
 * @param {string|object} input - Markdown string or parsed AST
 * @param {RenderOptions} [options={}] - Rendering options
 * @returns {string} The rendered HTML
 */
export function renderHtmlSync(input, options = {}) {
  // This will throw if WASM is not initialized
  const wasm = globalThis.__fmd_wasm_instance

  if (!wasm) {
    throw new Error('WASM not initialized. Call renderHtml() or initWasm() first.')
  }

  try {
    if (typeof input === 'string') {
      // Pass options object directly, not JSON string
      return wasm.renderHtml(input, options)
    }

    if (typeof input === 'object' && input !== null) {
      if (wasm.renderAstToHtml) {
        return wasm.renderAstToHtml(JSON.stringify(input), JSON.stringify(options))
      }
      throw new Error('AST rendering not yet implemented in WASM module')
    }

    throw new TypeError('Input must be a string (markdown) or object (AST)')
  } catch (error) {
    const enhancedError = new Error(`Render error: ${error.message}`)
    enhancedError.originalError = error
    throw enhancedError
  }
}

/**
 * Sanitize HTML string
 * @param {string} html - HTML to sanitize
 * @param {Object} [options={}] - Sanitization options
 * @returns {Promise<string>} Sanitized HTML
 */
export async function sanitizeHtml(html, options = {}) {
  const wasm = await getWasmInstance()

  if (!wasm.sanitizeHtml) {
    // Fallback to basic sanitization if WASM doesn't provide it
    return html
      .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '')
      .replace(/on\w+\s*=\s*"[^"]*"/gi, '')
      .replace(/on\w+\s*=\s*'[^']*'/gi, '')
      .replace(/javascript:/gi, '')
  }

  try {
    return wasm.sanitizeHtml(html, JSON.stringify(options))
  } catch (error) {
    throw new Error(`Sanitization error: ${error.message}`)
  }
}

/**
 * Quick render with default safe options
 * @param {string} markdown - Markdown content
 * @returns {Promise<string>} Safe HTML output
 */
export async function quickRender(markdown) {
  return renderHtml(markdown, {
    sanitize: true,
    allowDangerousHtml: false,
    gfm: true,
  })
}

/**
 * Render with custom renderer functions
 * @param {string|object} input - Markdown or AST
 * @param {Object} renderers - Custom renderer functions
 * @returns {Promise<string>} Rendered HTML
 */
export async function renderWithCustom(input, renderers = {}) {
  // First parse if needed
  const ast = typeof input === 'string' ? await parse(input) : input

  // Apply custom renderers by walking the AST
  // This is a simplified version - full implementation would
  // properly walk and transform the tree
  const transformed = walkAndTransform(ast, renderers)

  // Render the transformed AST
  return renderHtml(transformed)
}

/**
 * Walk and transform AST with custom renderers
 * @private
 */
function walkAndTransform(node, renderers) {
  // This is a placeholder - full implementation would properly
  // traverse and transform the AST based on custom renderers
  if (renderers[node.type]) {
    return renderers[node.type](node)
  }

  if (node.children) {
    node.children = node.children.map((child) => walkAndTransform(child, renderers))
  }

  return node
}
