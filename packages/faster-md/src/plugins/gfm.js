// GFM (GitHub Flavored Markdown) plugin for faster-md
// Adds support for GFM extensions like tables, strikethrough, autolinks, task lists

import { createPlugin } from './registry.js'

/**
 * GFM plugin options
 * @typedef {Object} GfmOptions
 * @property {boolean} [tables=true] - Enable tables
 * @property {boolean} [strikethrough=true] - Enable strikethrough
 * @property {boolean} [autolinks=true] - Enable autolinks
 * @property {boolean} [taskLists=true] - Enable task lists
 * @property {boolean} [footnotes=true] - Enable footnotes
 * @property {boolean} [tagFilter=true] - Enable HTML tag filtering
 */

/**
 * Create GFM plugin
 * @param {GfmOptions} options - Plugin options
 * @returns {import('./registry.js').Plugin} GFM plugin
 */
export function gfmPlugin(options = {}) {
  const {
    tables = true,
    strikethrough = true,
    autolinks = true,
    taskLists = true,
    footnotes = true,
    tagFilter = true,
  } = options

  return createPlugin('gfm')
    .version('1.0.0')
    .options({
      tables,
      strikethrough,
      autolinks,
      taskLists,
      footnotes,
      tagFilter,
    })
    .parser((markdown, processor) => {
      // Pre-process GFM syntax before main parser
      let result = markdown

      // Process task lists: - [ ] and - [x]
      if (taskLists) {
        result = processTaskLists(result)
      }

      // Process footnotes [^1]
      if (footnotes) {
        result = processFootnotes(result)
      }

      return result
    })
    .transformer((ast, processor) => {
      // Transform AST for GFM features
      return transformGfmAst(ast, {
        tables,
        strikethrough,
        autolinks,
        taskLists,
        footnotes,
        tagFilter,
      })
    })
    .compiler((ast, processor) => {
      // Add GFM-specific rendering
      return ast
    })
    .build()
}

/**
 * Process task list syntax
 */
function processTaskLists(markdown) {
  // Convert - [ ] to checkbox syntax
  return markdown
    .replace(/^(\s*)-\s+\[x\]/gim, '$1- [x] ')
    .replace(/^(\s*)-\s+\[\s\]/gim, '$1- [ ] ')
}

/**
 * Process footnotes
 */
function processFootnotes(markdown) {
  const footnoteRefs = new Map()
  const footnoteDefinitions = new Map()
  
  // Find footnote references [^1]
  let refCounter = 0
  markdown = markdown.replace(/\[\^([^\]]+)\]/g, (match, id) => {
    refCounter++
    footnoteRefs.set(id, refCounter)
    return `[^${refCounter}]`
  })

  // Find footnote definitions
  const lines = markdown.split('\n')
  const processedLines = []
  let inFootnote = false
  let currentFootnote = null

  for (const line of lines) {
    const footnoteDef = line.match(/^\[\^([^\]]+)\]:\s*(.*)/)
    
    if (footnoteDef) {
      const [, id, content] = footnoteDef
      currentFootnote = id
      inFootnote = true
      
      if (!footnoteDefinitions.has(id)) {
        footnoteDefinitions.set(id, [])
      }
      footnoteDefinitions.get(id).push(content)
    } else if (inFootnote && line.match(/^\s+/)) {
      // Continuation of footnote
      footnoteDefinitions.get(currentFootnote).push(line.trim())
    } else {
      inFootnote = false
      currentFootnote = null
      processedLines.push(line)
    }
  }

  // Append footnotes at the end
  if (footnoteDefinitions.size > 0) {
    processedLines.push('')
    processedLines.push('---')
    processedLines.push('')
    
    for (const [id, lines] of footnoteDefinitions) {
      const num = footnoteRefs.get(id)
      processedLines.push(`[^${num}]: ${lines.join(' ')}`)
    }
  }

  return processedLines.join('\n')
}

/**
 * Transform AST for GFM features
 */
function transformGfmAst(ast, options) {
  const walk = (node) => {
    if (!node) return node

    // Transform based on node type
    switch (node.type) {
      case 'paragraph':
        node = transformParagraph(node, options)
        break

      case 'list':
        node = transformList(node, options)
        break

      case 'text':
        node = transformText(node, options)
        break

      case 'html':
        if (options.tagFilter) {
          node = filterHtmlTags(node)
        }
        break
    }

    // Recursively transform children
    if (node.children && Array.isArray(node.children)) {
      node.children = node.children.map(walk)
    }

    return node
  }

  return walk(ast)
}

/**
 * Transform paragraph for GFM features
 */
function transformParagraph(node, options) {
  if (!node.children) return node

  const newChildren = []
  
  for (let child of node.children) {
    if (child.type === 'text') {
      // Process strikethrough ~~text~~
      if (options.strikethrough) {
        const parts = processStrikethrough(child.value)
        newChildren.push(...parts)
      }
      // Process autolinks
      else if (options.autolinks) {
        const parts = processAutolinks(child.value)
        newChildren.push(...parts)
      } else {
        newChildren.push(child)
      }
    } else {
      newChildren.push(child)
    }
  }

  node.children = newChildren
  return node
}

/**
 * Process strikethrough syntax
 */
function processStrikethrough(text) {
  const parts = []
  const regex = /~~([^~]+)~~/g
  let lastIndex = 0
  let match

  while ((match = regex.exec(text)) !== null) {
    // Add text before strikethrough
    if (match.index > lastIndex) {
      parts.push({
        type: 'text',
        value: text.slice(lastIndex, match.index),
      })
    }

    // Add strikethrough node
    parts.push({
      type: 'delete',
      children: [{
        type: 'text',
        value: match[1],
      }],
    })

    lastIndex = regex.lastIndex
  }

  // Add remaining text
  if (lastIndex < text.length) {
    parts.push({
      type: 'text',
      value: text.slice(lastIndex),
    })
  }

  return parts.length > 0 ? parts : [{ type: 'text', value: text }]
}

/**
 * Process autolinks
 */
function processAutolinks(text) {
  const parts = []
  // URL regex - simplified version
  const urlRegex = /\b(https?:\/\/[^\s<]+[^<.,:;"')\]\s])/g
  // Email regex
  const emailRegex = /\b([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})\b/g
  
  let lastIndex = 0
  const matches = []

  // Find all URLs
  let match
  while ((match = urlRegex.exec(text)) !== null) {
    matches.push({
      index: match.index,
      length: match[0].length,
      type: 'url',
      value: match[0],
    })
  }

  // Find all emails
  while ((match = emailRegex.exec(text)) !== null) {
    matches.push({
      index: match.index,
      length: match[0].length,
      type: 'email',
      value: match[0],
    })
  }

  // Sort matches by index
  matches.sort((a, b) => a.index - b.index)

  // Process matches
  for (const match of matches) {
    // Add text before match
    if (match.index > lastIndex) {
      parts.push({
        type: 'text',
        value: text.slice(lastIndex, match.index),
      })
    }

    // Add link node
    parts.push({
      type: 'link',
      url: match.type === 'email' ? `mailto:${match.value}` : match.value,
      children: [{
        type: 'text',
        value: match.value,
      }],
    })

    lastIndex = match.index + match.length
  }

  // Add remaining text
  if (lastIndex < text.length) {
    parts.push({
      type: 'text',
      value: text.slice(lastIndex),
    })
  }

  return parts.length > 0 ? parts : [{ type: 'text', value: text }]
}

/**
 * Transform list for task lists
 */
function transformList(node, options) {
  if (!options.taskLists || !node.children) return node

  for (const item of node.children) {
    if (item.type === 'listItem' && item.children && item.children.length > 0) {
      const firstChild = item.children[0]
      
      if (firstChild.type === 'paragraph' && firstChild.children && firstChild.children.length > 0) {
        const textNode = firstChild.children[0]
        
        if (textNode.type === 'text') {
          // Check for task list syntax
          const taskMatch = textNode.value.match(/^\[([ x])\]\s+(.*)/)
          
          if (taskMatch) {
            const [, checked, text] = taskMatch
            
            // Add task list properties
            item.checked = checked === 'x'
            item.task = true
            
            // Update text
            textNode.value = text
          }
        }
      }
    }
  }

  return node
}

/**
 * Transform text nodes for GFM features
 */
function transformText(node, options) {
  // Text transformation happens in paragraph processing
  return node
}

/**
 * Filter dangerous HTML tags
 */
function filterHtmlTags(node) {
  const dangerousTags = [
    'script',
    'style',
    'iframe',
    'object',
    'embed',
    'form',
    'input',
    'textarea',
    'button',
    'select',
  ]

  if (node.value) {
    // Remove dangerous tags
    for (const tag of dangerousTags) {
      const regex = new RegExp(`</?${tag}[^>]*>`, 'gi')
      node.value = node.value.replace(regex, '')
    }
  }

  return node
}

// Default export
export default gfmPlugin