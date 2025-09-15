// faster-md - High-performance Markdown processor with WASM
export const VERSION = '0.1.0'

// Placeholder exports - will be implemented when WASM is built
export function renderHtml(markdown: string, _options?: unknown): string {
  // TODO: Load and use WASM module
  return `<p>Placeholder output for: ${markdown}</p>`
}

export interface AstNode {
  type: string
  children: AstNode[]
}

export function parseToAst(_markdown: string, _options?: unknown): AstNode {
  // TODO: Load and use WASM module
  return {
    type: 'root',
    children: [],
  }
}
