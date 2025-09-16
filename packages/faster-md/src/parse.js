// Parse wrapper for faster-md
// Wraps the WASM parseToAst function with JavaScript-friendly interface

import { getWasmInstance } from './loader.js'

/**
 * Parse options
 * @typedef {Object} ParseOptions
 * @property {boolean} [gfm=false] - Enable GitHub Flavored Markdown
 * @property {boolean} [frontmatter=false] - Enable frontmatter parsing
 * @property {boolean} [mdx=false] - Enable MDX parsing
 * @property {boolean} [position=true] - Include position information
 * @property {string} [sessionId] - Session ID for incremental parsing
 */

/**
 * Parse Markdown to AST
 * @param {string} markdown - The markdown content to parse
 * @param {ParseOptions} [options={}] - Parsing options
 * @returns {Promise<object>} The parsed AST
 */
export async function parse(markdown, options = {}) {
  if (typeof markdown !== 'string') {
    throw new TypeError('Markdown content must be a string')
  }

  const wasm = await getWasmInstance()

  try {
    // Call the WASM parseToAst function - pass options object directly
    const resultJson = wasm.parseToAst(markdown, options)
    
    // Parse the JSON string returned by WASM
    const result = typeof resultJson === 'string' ? JSON.parse(resultJson) : resultJson

    // Return the result or just the AST if requested
    return result
  } catch (error) {
    // Enhance error with context
    const enhancedError = new Error(`Parse error: ${error.message}`)
    enhancedError.originalError = error
    enhancedError.markdown = markdown.substring(0, 100) + (markdown.length > 100 ? '...' : '')
    throw enhancedError
  }
}

/**
 * Parse Markdown synchronously (requires WASM to be pre-initialized)
 * @param {string} markdown - The markdown content to parse
 * @param {ParseOptions} [options={}] - Parsing options
 * @returns {object} The parsed AST
 */
export function parseSync(markdown, options = {}) {
  if (typeof markdown !== 'string') {
    throw new TypeError('Markdown content must be a string')
  }

  // This will throw if WASM is not initialized
  const wasm = globalThis.__fmd_wasm_instance

  if (!wasm) {
    throw new Error('WASM not initialized. Call parse() or initWasm() first.')
  }

  try {
    // Pass options object directly
    const resultJson = wasm.parseToAst(markdown, options)
    // Parse the JSON string if needed
    return typeof resultJson === 'string' ? JSON.parse(resultJson) : resultJson
  } catch (error) {
    const enhancedError = new Error(`Parse error: ${error.message}`)
    enhancedError.originalError = error
    enhancedError.markdown = markdown.substring(0, 100) + (markdown.length > 100 ? '...' : '')
    throw enhancedError
  }
}

/**
 * Create a parse session for incremental parsing
 * @param {string} [sessionId] - Optional session ID
 * @returns {Promise<string>} The session ID
 */
export async function createParseSession(sessionId) {
  const wasm = await getWasmInstance()

  try {
    // Create a new session manager if needed
    if (!wasm.sessionManager) {
      wasm.sessionManager = new wasm.SessionManager()
    }

    return wasm.sessionManager.createSession(sessionId)
  } catch (error) {
    throw new Error(`Failed to create parse session: ${error.message}`)
  }
}

/**
 * Parse incrementally using a session
 * @param {string} sessionId - The session ID
 * @param {string} markdown - The markdown content
 * @param {object} [edits] - Edit operations for incremental parsing
 * @returns {Promise<object>} The parsed AST
 */
export async function parseIncremental(sessionId, markdown, edits) {
  const wasm = await getWasmInstance()

  if (!wasm.sessionManager) {
    throw new Error('No session manager available')
  }

  try {
    const options = {
      sessionId,
      incremental: true,
      edits: edits || [],
    }

    const astJson = wasm.parseToAst(markdown, JSON.stringify(options))
    return JSON.parse(astJson)
  } catch (error) {
    throw new Error(`Incremental parse error: ${error.message}`)
  }
}

/**
 * Clear a parse session
 * @param {string} sessionId - The session ID to clear
 * @returns {Promise<void>}
 */
export async function clearSession(sessionId) {
  const wasm = await getWasmInstance()

  if (wasm.sessionManager) {
    wasm.sessionManager.clearSession(sessionId)
  }
}
