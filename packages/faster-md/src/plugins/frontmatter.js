// Frontmatter plugin for faster-md
// Parses YAML/JSON frontmatter from markdown documents

import matter from 'gray-matter'
import { createPlugin } from './registry.js'

/**
 * Frontmatter plugin options
 * @typedef {Object} FrontmatterOptions
 * @property {string[]} [delimiters=['---']] - Frontmatter delimiters
 * @property {Function} [parser] - Custom parser function
 * @property {boolean} [excerpt=false] - Extract excerpt
 * @property {string|Function} [excerpt_separator='---'] - Excerpt separator
 * @property {boolean} [stripFrontmatter=true] - Remove frontmatter from content
 * @property {Function} [transform] - Transform parsed frontmatter
 */

/**
 * Create frontmatter plugin
 * @param {FrontmatterOptions} options - Plugin options
 * @returns {import('./registry.js').Plugin} Frontmatter plugin
 */
export function frontmatterPlugin(options = {}) {
  const {
    delimiters = ['---'],
    parser,
    excerpt = false,
    excerpt_separator = '---',
    stripFrontmatter = true,
    transform,
  } = options

  return createPlugin('frontmatter')
    .version('1.0.0')
    .options(options)
    .parser((markdown, processor) => {
      // Parse frontmatter using gray-matter
      const matterOptions = {
        delimiters,
        excerpt,
        excerpt_separator,
      }

      if (parser) {
        matterOptions.engines = {
          custom: parser,
        }
      }

      const parsed = matter(markdown, matterOptions)

      // Transform frontmatter if needed
      let frontmatterData = parsed.data
      if (transform && typeof transform === 'function') {
        frontmatterData = transform(frontmatterData)
      }

      // Store frontmatter in processor data
      processor.data('frontmatter', frontmatterData)

      if (parsed.excerpt) {
        processor.data('excerpt', parsed.excerpt)
      }

      // Return content with or without frontmatter
      return stripFrontmatter ? parsed.content : markdown
    })
    .transformer((ast, processor) => {
      const frontmatter = processor.data('frontmatter')
      const excerpt = processor.data('excerpt')

      if (!frontmatter && !excerpt) {
        return ast
      }

      // Add frontmatter node to AST
      const frontmatterNode = {
        type: 'frontmatter',
        value: frontmatter,
        data: {
          frontmatter,
          excerpt,
        },
      }

      // Ensure AST has proper structure
      if (!ast.children) {
        ast.children = []
      }

      // Insert frontmatter node at the beginning
      ast.children.unshift(frontmatterNode)

      // Process frontmatter directives
      ast = processFrontmatterDirectives(ast, frontmatter)

      return ast
    })
    .compiler((ast, processor) => {
      // Frontmatter doesn't render to HTML by default
      // But we can add metadata comments or data attributes
      const frontmatter = processor.data('frontmatter')

      if (frontmatter?.renderMeta) {
        return addMetadataToHtml(ast, frontmatter)
      }

      return ast
    })
    .build()
}

/**
 * Process frontmatter directives that affect rendering
 */
function processFrontmatterDirectives(ast, frontmatter) {
  if (!frontmatter) return ast

  // Handle title
  if (frontmatter.title && !hasHeading(ast)) {
    ast.children.splice(1, 0, {
      type: 'heading',
      depth: 1,
      children: [
        {
          type: 'text',
          value: frontmatter.title,
        },
      ],
    })
  }

  // Handle table of contents
  if (frontmatter.toc) {
    const toc = generateTableOfContents(ast)
    if (toc) {
      const tocPosition = frontmatter.tocPosition || 'after-title'
      const insertIndex = tocPosition === 'top' ? 1 : 2

      ast.children.splice(insertIndex, 0, toc)
    }
  }

  // Handle custom classes
  if (frontmatter.className) {
    ast.data = ast.data || {}
    ast.data.className = frontmatter.className
  }

  // Handle custom attributes
  if (frontmatter.attributes) {
    ast.data = ast.data || {}
    ast.data.attributes = frontmatter.attributes
  }

  return ast
}

/**
 * Check if AST has a heading
 */
function hasHeading(ast) {
  if (!ast.children) return false

  return ast.children.some((node) => node.type === 'heading' && node.depth === 1)
}

/**
 * Generate table of contents from AST
 */
function generateTableOfContents(ast) {
  const headings = extractHeadings(ast)

  if (headings.length === 0) {
    return null
  }

  const tocItems = headings.map((heading) => ({
    type: 'listItem',
    children: [
      {
        type: 'link',
        url: `#${slugify(heading.text)}`,
        children: [
          {
            type: 'text',
            value: heading.text,
          },
        ],
      },
    ],
    depth: heading.depth - 1,
  }))

  return {
    type: 'section',
    data: {
      className: 'table-of-contents',
    },
    children: [
      {
        type: 'heading',
        depth: 2,
        children: [
          {
            type: 'text',
            value: 'Table of Contents',
          },
        ],
      },
      {
        type: 'list',
        ordered: false,
        children: tocItems,
      },
    ],
  }
}

/**
 * Extract headings from AST
 */
function extractHeadings(ast) {
  const headings = []

  const walk = (node) => {
    if (!node) return

    if (node.type === 'heading' && node.depth >= 2 && node.depth <= 3) {
      const text = extractText(node)
      if (text) {
        headings.push({
          depth: node.depth,
          text,
        })
      }
    }

    if (node.children && Array.isArray(node.children)) {
      node.children.forEach(walk)
    }
  }

  walk(ast)
  return headings
}

/**
 * Extract text from a node
 */
function extractText(node) {
  if (node.type === 'text') {
    return node.value
  }

  if (node.children && Array.isArray(node.children)) {
    return node.children.map(extractText).join('')
  }

  return ''
}

/**
 * Create a slug from text
 */
function slugify(text) {
  return text
    .toLowerCase()
    .replace(/[^\w\s-]/g, '') // Remove special characters
    .replace(/\s+/g, '-') // Replace spaces with hyphens
    .replace(/--+/g, '-') // Replace multiple hyphens with single
    .replace(/^-+|-+$/g, '') // Trim hyphens from start and end
}

/**
 * Add metadata to HTML output
 */
function addMetadataToHtml(ast, frontmatter) {
  // Add metadata as data attributes
  if (ast.type === 'root' && ast.children && ast.children.length > 0) {
    const wrapper = {
      type: 'element',
      tagName: 'div',
      properties: {
        'data-frontmatter': JSON.stringify(frontmatter),
      },
      children: ast.children,
    }

    // Add specific frontmatter fields as data attributes
    if (frontmatter.title) {
      wrapper.properties['data-title'] = frontmatter.title
    }
    if (frontmatter.date) {
      wrapper.properties['data-date'] = frontmatter.date
    }
    if (frontmatter.author) {
      wrapper.properties['data-author'] = frontmatter.author
    }
    if (frontmatter.tags) {
      wrapper.properties['data-tags'] = Array.isArray(frontmatter.tags)
        ? frontmatter.tags.join(',')
        : frontmatter.tags
    }

    ast.children = [wrapper]
  }

  return ast
}

/**
 * Create preset frontmatter configurations
 */
export const presets = {
  blog: {
    excerpt: true,
    excerpt_separator: '<!-- more -->',
    transform: (data) => ({
      ...data,
      date: data.date ? new Date(data.date).toISOString() : null,
      tags: data.tags ? (Array.isArray(data.tags) ? data.tags : [data.tags]) : [],
      draft: data.draft || false,
    }),
  },

  documentation: {
    excerpt: false,
    transform: (data) => ({
      ...data,
      toc: data.toc !== false, // Enable by default
      editUrl: data.editUrl || null,
      lastModified: data.lastModified || null,
    }),
  },

  minimal: {
    stripFrontmatter: true,
    excerpt: false,
  },
}

/**
 * Create frontmatter plugin with preset
 */
export function frontmatterWithPreset(preset, options = {}) {
  const presetConfig = typeof preset === 'string' ? presets[preset] : preset

  if (!presetConfig) {
    throw new Error(`Unknown frontmatter preset: ${preset}`)
  }

  return frontmatterPlugin({
    ...presetConfig,
    ...options,
  })
}

// Default export
export default frontmatterPlugin
