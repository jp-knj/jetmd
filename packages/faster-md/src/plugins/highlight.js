// Syntax highlighting plugin for faster-md
// Adds syntax highlighting to code blocks

import { createPlugin } from './registry.js'

/**
 * Highlight plugin options
 * @typedef {Object} HighlightOptions
 * @property {Function} [highlighter] - Custom highlighter function
 * @property {string} [theme='github'] - Color theme
 * @property {boolean} [lineNumbers=false] - Show line numbers
 * @property {boolean} [wrapLines=false] - Wrap each line in span
 * @property {string} [defaultLanguage='text'] - Default language
 * @property {Object} [aliases={}] - Language aliases
 * @property {boolean} [inline=true] - Highlight inline code
 * @property {string[]} [languages] - Supported languages
 */

/**
 * Create syntax highlighting plugin
 * @param {HighlightOptions} options - Plugin options
 * @returns {import('./registry.js').Plugin} Highlight plugin
 */
export function highlightPlugin(options = {}) {
  const {
    highlighter,
    theme = 'github',
    lineNumbers = false,
    wrapLines = false,
    defaultLanguage = 'text',
    aliases = {},
    inline = true,
    languages = [],
  } = options

  // Language alias map
  const languageAliases = {
    js: 'javascript',
    ts: 'typescript',
    py: 'python',
    rb: 'ruby',
    sh: 'bash',
    yml: 'yaml',
    ...aliases,
  }

  return createPlugin('highlight')
    .version('1.0.0')
    .options(options)
    .transformer((ast, _processor) => {
      // Walk the AST and highlight code blocks
      return walkAndHighlight(ast, {
        highlighter,
        theme,
        lineNumbers,
        wrapLines,
        defaultLanguage,
        languageAliases,
        inline,
        languages,
      })
    })
    .build()
}

/**
 * Walk AST and apply syntax highlighting
 */
function walkAndHighlight(node, options) {
  if (!node) return node

  // Handle code blocks
  if (node.type === 'code') {
    return highlightCodeBlock(node, options)
  }

  // Handle inline code if enabled
  if (node.type === 'inlineCode' && options.inline) {
    return highlightInlineCode(node, options)
  }

  // Recursively process children
  if (node.children && Array.isArray(node.children)) {
    node.children = node.children.map((child) => walkAndHighlight(child, options))
  }

  return node
}

/**
 * Highlight a code block
 */
function highlightCodeBlock(node, options) {
  const {
    highlighter,
    theme,
    lineNumbers,
    wrapLines,
    defaultLanguage,
    languageAliases,
    languages,
  } = options

  // Get language
  let language = node.lang || node.language || defaultLanguage
  language = languageAliases[language] || language

  // Check if language is supported
  if (languages.length > 0 && !languages.includes(language)) {
    return node // Return unchanged if not supported
  }

  // Apply highlighting
  let highlightedCode

  if (highlighter && typeof highlighter === 'function') {
    // Use custom highlighter
    highlightedCode = highlighter(node.value, language, options)
  } else {
    // Use built-in highlighting
    highlightedCode = builtinHighlight(node.value, language, theme)
  }

  // Add line numbers if requested
  if (lineNumbers) {
    highlightedCode = addLineNumbers(highlightedCode, wrapLines)
  }

  // Create highlighted node
  return {
    type: 'html',
    value: `<pre class="language-${language}"><code class="language-${language}">${highlightedCode}</code></pre>`,
    data: {
      ...node.data,
      highlighted: true,
      language,
    },
  }
}

/**
 * Highlight inline code
 */
function highlightInlineCode(node, options) {
  const { highlighter, theme, defaultLanguage } = options

  // Try to detect language from context or use default
  const language = detectInlineLanguage(node) || defaultLanguage

  let highlightedCode

  if (highlighter && typeof highlighter === 'function') {
    highlightedCode = highlighter(node.value, language, { ...options, inline: true })
  } else {
    highlightedCode = builtinHighlightInline(node.value, language, theme)
  }

  return {
    type: 'html',
    value: `<code class="language-${language} inline">${highlightedCode}</code>`,
    data: {
      ...node.data,
      highlighted: true,
      language,
    },
  }
}

/**
 * Built-in syntax highlighting (basic)
 */
function builtinHighlight(code, language, theme) {
  // This is a simplified highlighter
  // In production, you would use Prism.js, highlight.js, or shiki

  const themes = {
    github: {
      keyword: 'color: #d73a49',
      string: 'color: #032f62',
      comment: 'color: #6a737d',
      function: 'color: #6f42c1',
      number: 'color: #005cc5',
    },
    dark: {
      keyword: 'color: #c678dd',
      string: 'color: #98c379',
      comment: 'color: #5c6370',
      function: 'color: #61afef',
      number: 'color: #d19a66',
    },
  }

  const colors = themes[theme] || themes.github

  // Language-specific patterns
  const patterns = {
    javascript: [
      {
        regex:
          /\b(const|let|var|function|return|if|else|for|while|class|extends|import|export|from|async|await)\b/g,
        class: 'keyword',
      },
      { regex: /(["'`])(?:(?=(\\?))\2.)*?\1/g, class: 'string' },
      { regex: /\/\/.*$/gm, class: 'comment' },
      { regex: /\/\*[\s\S]*?\*\//g, class: 'comment' },
      { regex: /\b\d+(\.\d+)?\b/g, class: 'number' },
      { regex: /\b([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\(/g, class: 'function' },
    ],
    python: [
      {
        regex:
          /\b(def|class|if|else|elif|for|while|return|import|from|as|try|except|finally|with|async|await)\b/g,
        class: 'keyword',
      },
      { regex: /(["'])(?:(?=(\\?))\2.)*?\1/g, class: 'string' },
      { regex: /#.*$/gm, class: 'comment' },
      { regex: /\b\d+(\.\d+)?\b/g, class: 'number' },
      { regex: /\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/g, class: 'function' },
    ],
    html: [
      { regex: /<!--[\s\S]*?-->/g, class: 'comment' },
      { regex: /<\/?[a-zA-Z][^>]*>/g, class: 'keyword' },
      { regex: /(["'])(?:(?=(\\?))\2.)*?\1/g, class: 'string' },
    ],
    css: [
      { regex: /\/\*[\s\S]*?\*\//g, class: 'comment' },
      { regex: /[.#]?[a-zA-Z][\w-]*/g, class: 'keyword' },
      { regex: /(["'])(?:(?=(\\?))\2.)*?\1/g, class: 'string' },
      { regex: /\b\d+(\.\d+)?(px|em|rem|%|vh|vw)?\b/g, class: 'number' },
    ],
  }

  // Get patterns for language
  const langPatterns = patterns[language] || []

  // Escape HTML
  let highlighted = escapeHtml(code)

  // Apply highlighting
  for (const pattern of langPatterns) {
    const style = colors[pattern.class] || ''
    highlighted = highlighted.replace(
      pattern.regex,
      (match) => `<span style="${style}">${match}</span>`,
    )
  }

  return highlighted
}

/**
 * Built-in inline highlighting
 */
function builtinHighlightInline(code, _language, _theme) {
  // Simpler highlighting for inline code
  return `<span class="inline-code">${escapeHtml(code)}</span>`
}

/**
 * Add line numbers to code
 */
function addLineNumbers(code, wrapLines) {
  const lines = code.split('\n')

  return lines
    .map((line, index) => {
      const lineNumber = index + 1
      const lineClass = `line line-${lineNumber}`

      if (wrapLines) {
        return `<span class="${lineClass}"><span class="line-number">${lineNumber}</span>${line}</span>`
      }
      return `<span class="line-number">${lineNumber}</span>${line}`
    })
    .join('\n')
}

/**
 * Detect language for inline code
 */
function detectInlineLanguage(node) {
  // Simple heuristics for language detection
  const value = node.value

  // Check for common patterns
  if (/^(const|let|var|function)\s/.test(value)) return 'javascript'
  if (/^(def|class|import)\s/.test(value)) return 'python'
  if (/^<[a-zA-Z]/.test(value)) return 'html'
  if (/^[.#][a-zA-Z]/.test(value)) return 'css'
  if (/^\$/.test(value)) return 'bash'

  return null
}

/**
 * Escape HTML characters
 */
function escapeHtml(str) {
  const htmlEscapes = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#39;',
  }

  return str.replace(/[&<>"']/g, (char) => htmlEscapes[char])
}

/**
 * Presets for popular highlighting libraries
 */
export const presets = {
  prism: {
    highlighter: null, // Would be Prism.highlight
    theme: 'prism',
    lineNumbers: false,
    inline: true,
  },

  highlightjs: {
    highlighter: null, // Would be hljs.highlight
    theme: 'github',
    lineNumbers: false,
    inline: false,
  },

  shiki: {
    highlighter: null, // Would be shiki.getHighlighter
    theme: 'github-dark',
    lineNumbers: true,
    inline: true,
  },
}

/**
 * Create highlight plugin with preset
 */
export function highlightWithPreset(preset, options = {}) {
  const presetConfig = typeof preset === 'string' ? presets[preset] : preset

  if (!presetConfig) {
    throw new Error(`Unknown highlight preset: ${preset}`)
  }

  return highlightPlugin({
    ...presetConfig,
    ...options,
  })
}

// Default export
export default highlightPlugin
