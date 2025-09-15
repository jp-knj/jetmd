// Vite plugin for faster-md
// Transforms .md files into HTML or React components

import { readFile } from 'node:fs/promises'
import { createFilter } from '@rollup/pluginutils'
import { renderHtml, parse, createProcessor } from 'faster-md'
import matter from 'gray-matter'

/**
 * Vite plugin options
 * @typedef {Object} VitePluginOptions
 * @property {string[]} [include] - Files to include (default: ['**/*.md'])
 * @property {string[]} [exclude] - Files to exclude (default: ['**/node_modules/**'])
 * @property {boolean} [gfm] - Enable GitHub Flavored Markdown (default: true)
 * @property {boolean} [frontmatter] - Enable frontmatter parsing (default: true)
 * @property {boolean} [sanitize] - Enable HTML sanitization (default: true)
 * @property {'html'|'react'|'vue'} [mode] - Output mode (default: 'html')
 * @property {boolean} [hmr] - Enable HMR (default: true)
 * @property {Function} [transformHtml] - Custom HTML transformer
 * @property {Function} [transformAst] - Custom AST transformer
 * @property {Object} [processorOptions] - Options for the processor
 */

/**
 * Create Vite plugin for Markdown processing
 * @param {VitePluginOptions} options - Plugin options
 * @returns {import('vite').Plugin} Vite plugin
 */
export function fasterMdPlugin(options = {}) {
  const {
    include = ['**/*.md'],
    exclude = ['**/node_modules/**'],
    gfm = true,
    frontmatter = true,
    sanitize = true,
    mode = 'html',
    hmr = true,
    transformHtml,
    transformAst,
    processorOptions = {},
  } = options

  const filter = createFilter(include, exclude)
  const processor = createProcessor({
    gfm,
    frontmatter,
    sanitize,
    ...processorOptions,
  })

  return {
    name: 'vite-plugin-faster-md',
    
    async transform(code, id) {
      if (!filter(id)) {
        return null
      }

      // Parse frontmatter if enabled
      let content = code
      let frontmatterData = {}
      
      if (frontmatter) {
        const parsed = matter(code)
        content = parsed.content
        frontmatterData = parsed.data
      }

      // Parse markdown to AST
      let ast = await parse(content, {
        gfm,
        position: true,
      })

      // Apply custom AST transformer if provided
      if (transformAst) {
        ast = await transformAst(ast, { id, frontmatter: frontmatterData })
      }

      // Generate HTML
      let html = await renderHtml(ast, {
        sanitize,
        xhtml: false,
      })

      // Apply custom HTML transformer if provided
      if (transformHtml) {
        html = await transformHtml(html, { id, frontmatter: frontmatterData })
      }

      // Generate output based on mode
      let output
      
      switch (mode) {
        case 'react':
          output = generateReactModule(html, frontmatterData, id)
          break
          
        case 'vue':
          output = generateVueModule(html, frontmatterData, id)
          break
          
        case 'html':
        default:
          output = generateHtmlModule(html, frontmatterData, id)
          break
      }

      return {
        code: output,
        map: null, // Source maps could be added here
      }
    },

    async handleHotUpdate({ file, server }) {
      if (!hmr || !filter(file)) {
        return
      }

      // Invalidate module
      const module = server.moduleGraph.getModuleById(file)
      if (module) {
        server.moduleGraph.invalidateModule(module)
      }

      // Send HMR update
      server.ws.send({
        type: 'custom',
        event: 'faster-md-update',
        data: {
          file,
        },
      })
    },

    configureServer(server) {
      // Add middleware for serving .md files directly in dev
      server.middlewares.use(async (req, res, next) => {
        if (!req.url?.endsWith('.md')) {
          return next()
        }

        try {
          const filePath = req.url.slice(1) // Remove leading slash
          const content = await readFile(filePath, 'utf-8')
          
          // Parse and render
          const html = await processor.process(content)
          
          res.setHeader('Content-Type', 'text/html')
          res.end(wrapInHtmlDocument(html))
        } catch (error) {
          next(error)
        }
      })
    },
  }
}

/**
 * Generate HTML module
 */
function generateHtmlModule(html, frontmatter, id) {
  const frontmatterExport = Object.keys(frontmatter).length > 0
    ? `export const frontmatter = ${JSON.stringify(frontmatter)};`
    : ''

  return `
${frontmatterExport}
export const html = ${JSON.stringify(html)};
export default html;

// HMR support
if (import.meta.hot) {
  import.meta.hot.accept()
  import.meta.hot.on('faster-md-update', (data) => {
    if (data.file === ${JSON.stringify(id)}) {
      import.meta.hot.invalidate()
    }
  })
}
`.trim()
}

/**
 * Generate React module
 */
function generateReactModule(html, frontmatter, id) {
  const frontmatterExport = Object.keys(frontmatter).length > 0
    ? `export const frontmatter = ${JSON.stringify(frontmatter)};`
    : ''

  return `
import React from 'react';

${frontmatterExport}

const MarkdownComponent = () => {
  return React.createElement('div', {
    className: 'markdown-content',
    dangerouslySetInnerHTML: { __html: ${JSON.stringify(html)} }
  });
};

MarkdownComponent.displayName = 'MarkdownContent';

export default MarkdownComponent;

// HMR support
if (import.meta.hot) {
  import.meta.hot.accept()
  import.meta.hot.on('faster-md-update', (data) => {
    if (data.file === ${JSON.stringify(id)}) {
      import.meta.hot.invalidate()
    }
  })
}
`.trim()
}

/**
 * Generate Vue module
 */
function generateVueModule(html, frontmatter, id) {
  const frontmatterData = Object.keys(frontmatter).length > 0
    ? JSON.stringify(frontmatter)
    : 'null'

  return `
<template>
  <div class="markdown-content" v-html="html"></div>
</template>

<script setup>
import { ref } from 'vue'

const html = ref(${JSON.stringify(html)})
const frontmatter = ${frontmatterData}

defineExpose({
  html,
  frontmatter
})
</script>

<style scoped>
.markdown-content {
  /* Markdown styles */
}
</style>
`.trim()
}

/**
 * Wrap HTML in a basic document for preview
 */
function wrapInHtmlDocument(html) {
  return `
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Markdown Preview</title>
  <style>
    body {
      max-width: 800px;
      margin: 0 auto;
      padding: 2rem;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      line-height: 1.6;
    }
    pre {
      background: #f6f8fa;
      padding: 1rem;
      border-radius: 6px;
      overflow-x: auto;
    }
    code {
      background: #f6f8fa;
      padding: 0.2em 0.4em;
      border-radius: 3px;
    }
  </style>
</head>
<body>
  ${html}
</body>
</html>
`.trim()
}

// Export additional utilities
export { createFilter }

// Default export
export default fasterMdPlugin