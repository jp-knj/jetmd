// faster-md - High-performance Markdown processor with WASM
export const VERSION = '0.1.0'

// Re-export all modules
export { getWasmInstance, resetWasm, isWasmInitialized } from './loader.js'
export {
  parse,
  parseSync,
  createParseSession,
  parseIncremental,
  clearSession,
} from './parse.js'
export {
  renderHtml,
  renderHtmlSync,
  sanitizeHtml,
  quickRender,
  renderWithCustom,
} from './render.js'
export {
  Processor,
  createProcessor,
  quickProcess,
} from './processor.js'
export {
  MarkdownStream,
  LineStream,
  DuplexMarkdownStream,
  createStream,
  createLineStream,
  createDuplexStream,
  processStream,
} from './stream.js'

// Export types
export interface AstNode {
  type: string
  children?: AstNode[]
  value?: string
  position?: {
    start: { line: number; column: number; offset: number }
    end: { line: number; column: number; offset: number }
  }
  data?: Record<string, unknown>
}

export interface ParseOptions {
  gfm?: boolean
  frontmatter?: boolean
  mdx?: boolean
  position?: boolean
  sessionId?: string
}

export interface RenderOptions {
  sanitize?: boolean
  allowDangerousHtml?: boolean
  gfm?: boolean
  xhtml?: boolean
  sanitizeOptions?: Record<string, unknown>
  sessionId?: string
}

export interface ProcessorOptions {
  gfm?: boolean
  frontmatter?: boolean
  sanitize?: boolean
  mdx?: boolean
  data?: Record<string, unknown>
}
