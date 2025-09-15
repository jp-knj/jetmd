// MDX tokenizer
// Detects and tokenizes MDX-specific syntax (JSX, ESM imports/exports)

/**
 * Token types
 */
export const TokenType = {
  TEXT: 'text',
  JSX_ELEMENT: 'jsx_element',
  JSX_EXPRESSION: 'jsx_expression',
  ESM_IMPORT: 'esm_import',
  ESM_EXPORT: 'esm_export',
  MARKDOWN: 'markdown',
  FRONTMATTER: 'frontmatter',
  COMMENT: 'comment',
}

/**
 * Token interface
 * @typedef {Object} Token
 * @property {string} type - Token type
 * @property {string} value - Token value
 * @property {number} start - Start position
 * @property {number} end - End position
 * @property {Object} [loc] - Location info
 */

/**
 * MDX tokenizer
 */
export class MDXTokenizer {
  constructor(options = {}) {
    this.options = {
      preserveNewlines: true,
      ...options,
    }
  }

  /**
   * Tokenize MDX content
   * @param {string} content - MDX content
   * @returns {Array<Token>} Array of tokens
   */
  tokenize(content) {
    const tokens = []
    let position = 0
    const length = content.length

    while (position < length) {
      // Check for frontmatter
      if (position === 0 && content.startsWith('---\n', position)) {
        const token = this.scanFrontmatter(content, position)
        if (token) {
          tokens.push(token)
          position = token.end
          continue
        }
      }

      // Check for ESM import
      if (this.isImport(content, position)) {
        const token = this.scanImport(content, position)
        if (token) {
          tokens.push(token)
          position = token.end
          continue
        }
      }

      // Check for ESM export
      if (this.isExport(content, position)) {
        const token = this.scanExport(content, position)
        if (token) {
          tokens.push(token)
          position = token.end
          continue
        }
      }

      // Check for JSX element
      if (this.isJsxElement(content, position)) {
        const token = this.scanJsxElement(content, position)
        if (token) {
          tokens.push(token)
          position = token.end
          continue
        }
      }

      // Check for JSX expression
      if (content[position] === '{' && !this.isEscaped(content, position)) {
        const token = this.scanJsxExpression(content, position)
        if (token) {
          tokens.push(token)
          position = token.end
          continue
        }
      }

      // Check for HTML comment
      if (content.startsWith('<!--', position)) {
        const token = this.scanComment(content, position)
        if (token) {
          tokens.push(token)
          position = token.end
          continue
        }
      }

      // Otherwise, scan markdown text
      const token = this.scanMarkdown(content, position)
      tokens.push(token)
      position = token.end
    }

    return tokens
  }

  /**
   * Check if position is at an import statement
   */
  isImport(content, position) {
    // Must be at line start
    if (position > 0 && content[position - 1] !== '\n') {
      return false
    }
    return content.startsWith('import ', position)
  }

  /**
   * Check if position is at an export statement
   */
  isExport(content, position) {
    // Must be at line start
    if (position > 0 && content[position - 1] !== '\n') {
      return false
    }
    return content.startsWith('export ', position)
  }

  /**
   * Check if position is at a JSX element
   */
  isJsxElement(content, position) {
    // Check for < followed by uppercase letter or component path
    if (content[position] !== '<') {
      return false
    }

    const next = content[position + 1]

    // Self-closing />
    if (next === '/') {
      return false
    }

    // Component must start with uppercase or be a member expression
    return /[A-Z]/.test(next) || /[a-z]/.test(next)
  }

  /**
   * Check if character is escaped
   */
  isEscaped(content, position) {
    let backslashes = 0
    let i = position - 1

    while (i >= 0 && content[i] === '\\') {
      backslashes++
      i--
    }

    return backslashes % 2 === 1
  }

  /**
   * Scan frontmatter
   */
  scanFrontmatter(content, start) {
    const endMarker = '\n---\n'
    const endIndex = content.indexOf(endMarker, start + 4)

    if (endIndex === -1) {
      return null
    }

    const end = endIndex + endMarker.length

    return {
      type: TokenType.FRONTMATTER,
      value: content.substring(start + 4, endIndex),
      start,
      end,
    }
  }

  /**
   * Scan import statement
   */
  scanImport(content, start) {
    let end = start

    // Find end of import (semicolon or newline)
    while (end < content.length) {
      if (content[end] === ';') {
        end++
        break
      }
      if (content[end] === '\n') {
        // Check if next line continues the import
        const nextLine = end + 1
        if (nextLine < content.length && /\s/.test(content[nextLine])) {
          end++
          continue
        }
        break
      }
      end++
    }

    return {
      type: TokenType.ESM_IMPORT,
      value: content.substring(start, end),
      start,
      end,
    }
  }

  /**
   * Scan export statement
   */
  scanExport(content, start) {
    let end = start
    let braceDepth = 0

    // Handle export default function() {} and similar
    while (end < content.length) {
      const ch = content[end]

      if (ch === '{') {
        braceDepth++
      } else if (ch === '}') {
        braceDepth--
        if (braceDepth === 0) {
          end++
          break
        }
      } else if (ch === ';' && braceDepth === 0) {
        end++
        break
      } else if (ch === '\n' && braceDepth === 0) {
        // Check if this is a complete statement
        const stmt = content.substring(start, end)
        if (stmt.includes('export default') || stmt.includes('export const')) {
          break
        }
      }

      end++
    }

    return {
      type: TokenType.ESM_EXPORT,
      value: content.substring(start, end),
      start,
      end,
    }
  }

  /**
   * Scan JSX element
   */
  scanJsxElement(content, start) {
    let end = start + 1
    let depth = 1
    let inString = false
    let stringChar = null

    // Get tag name
    const tagMatch = content.substring(start).match(/^<([A-Za-z][A-Za-z0-9.]*)/)
    if (!tagMatch) {
      return null
    }

    const tagName = tagMatch[1]

    // Scan for matching close tag or self-closing
    while (end < content.length && depth > 0) {
      const ch = content[end]

      // Handle strings
      if ((ch === '"' || ch === "'") && !this.isEscaped(content, end)) {
        if (!inString) {
          inString = true
          stringChar = ch
        } else if (ch === stringChar) {
          inString = false
          stringChar = null
        }
      }

      if (!inString) {
        // Check for self-closing
        if (ch === '/' && content[end + 1] === '>') {
          end += 2
          depth--
        }
        // Check for opening tag
        else if (ch === '<' && content[end + 1] !== '/') {
          const nextTag = content.substring(end).match(/^<([A-Za-z][A-Za-z0-9.]*)/)
          if (nextTag && nextTag[1] === tagName) {
            depth++
          }
        }
        // Check for closing tag
        else if (ch === '<' && content[end + 1] === '/') {
          const closeTag = content.substring(end).match(/^<\/([A-Za-z][A-Za-z0-9.]*)>/)
          if (closeTag && closeTag[1] === tagName) {
            end += closeTag[0].length
            depth--
            continue
          }
        }
      }

      end++
    }

    return {
      type: TokenType.JSX_ELEMENT,
      value: content.substring(start, end),
      start,
      end,
      tagName,
    }
  }

  /**
   * Scan JSX expression
   */
  scanJsxExpression(content, start) {
    let end = start + 1
    let braceDepth = 1
    let inString = false
    let stringChar = null

    while (end < content.length && braceDepth > 0) {
      const ch = content[end]

      // Handle strings
      if ((ch === '"' || ch === "'" || ch === '`') && !this.isEscaped(content, end)) {
        if (!inString) {
          inString = true
          stringChar = ch
        } else if (ch === stringChar) {
          inString = false
          stringChar = null
        }
      }

      if (!inString) {
        if (ch === '{') {
          braceDepth++
        } else if (ch === '}') {
          braceDepth--
        }
      }

      end++
    }

    return {
      type: TokenType.JSX_EXPRESSION,
      value: content.substring(start + 1, end - 1), // Exclude braces
      start,
      end,
    }
  }

  /**
   * Scan HTML comment
   */
  scanComment(content, start) {
    const endMarker = '-->'
    const end = content.indexOf(endMarker, start + 4)

    if (end === -1) {
      return null
    }

    return {
      type: TokenType.COMMENT,
      value: content.substring(start + 4, end),
      start,
      end: end + endMarker.length,
    }
  }

  /**
   * Scan markdown text
   */
  scanMarkdown(content, start) {
    let end = start

    // Scan until we hit MDX syntax
    while (end < content.length) {
      const ch = content[end]

      // Check for MDX syntax markers
      if (ch === '<' && this.isJsxElement(content, end)) {
        break
      }

      if (ch === '{' && !this.isEscaped(content, end)) {
        break
      }

      if (end === 0 || content[end - 1] === '\n') {
        if (this.isImport(content, end) || this.isExport(content, end)) {
          break
        }
      }

      if (content.startsWith('<!--', end)) {
        break
      }

      end++
    }

    // Ensure we captured at least one character
    if (end === start) {
      end = start + 1
    }

    return {
      type: TokenType.MARKDOWN,
      value: content.substring(start, end),
      start,
      end,
    }
  }
}

/**
 * Create a tokenizer instance
 * @param {Object} [options] - Tokenizer options
 * @returns {MDXTokenizer} Tokenizer instance
 */
export function createTokenizer(options) {
  return new MDXTokenizer(options)
}

/**
 * Quick tokenize
 * @param {string} content - MDX content
 * @returns {Array<Token>} Tokens
 */
export function tokenize(content) {
  const tokenizer = createTokenizer()
  return tokenizer.tokenize(content)
}
