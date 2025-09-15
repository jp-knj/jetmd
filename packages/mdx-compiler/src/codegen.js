// ESM code generator for MDX
// Generates ES modules from MDX tokens and AST

import { createJsxParser } from './jsx.js'

/**
 * Code generation options
 * @typedef {Object} CodegenOptions
 * @property {string} [pragma='React.createElement'] - JSX pragma
 * @property {string} [pragmaFrag='React.Fragment'] - Fragment pragma
 * @property {boolean} [development=false] - Development mode
 * @property {string} [jsxRuntime='classic'] - JSX runtime (classic or automatic)
 * @property {string} [jsxImportSource='react'] - JSX import source for automatic runtime
 * @property {boolean} [providerImportSource] - Import source for context provider
 */

/**
 * ESM code generator
 */
export class CodeGenerator {
  constructor(options = {}) {
    this.options = {
      pragma: 'React.createElement',
      pragmaFrag: 'React.Fragment',
      development: false,
      jsxRuntime: 'classic',
      jsxImportSource: 'react',
      ...options,
    }

    this.jsxParser = createJsxParser()
    this.imports = new Set()
    this.exports = new Map()
    this.components = new Set()
  }

  /**
   * Generate ES module from tokens
   * @param {Array} tokens - MDX tokens
   * @param {Object} metadata - Additional metadata
   * @returns {string} Generated ES module code
   */
  generate(tokens, metadata = {}) {
    this.reset()

    const parts = []

    // Add automatic JSX runtime import if needed
    if (this.options.jsxRuntime === 'automatic') {
      parts.push(this.generateAutomaticRuntimeImport())
    }

    // Process tokens and collect imports/exports
    const bodyParts = []

    for (const token of tokens) {
      switch (token.type) {
        case 'esm_import':
          this.collectImport(token.value)
          break

        case 'esm_export':
          this.collectExport(token.value)
          break

        case 'jsx_element':
          this.collectJsxComponents(token.value)
          bodyParts.push(this.wrapJsx(token.value))
          break

        case 'jsx_expression':
          bodyParts.push(this.wrapExpression(token.value))
          break

        case 'markdown':
          bodyParts.push(this.wrapMarkdown(token.value))
          break

        case 'frontmatter':
          // Handle frontmatter separately
          if (metadata.frontmatter !== false) {
            this.processFrontmatter(token.value)
          }
          break

        default:
          // Other tokens are passed through or ignored
          break
      }
    }

    // Generate imports section
    parts.push(this.generateImports())

    // Generate component function
    parts.push(this.generateComponent(bodyParts, metadata))

    // Generate exports section
    parts.push(this.generateExports())

    // Generate default export
    parts.push(this.generateDefaultExport(metadata))

    return parts.filter(Boolean).join('\n\n')
  }

  /**
   * Reset generator state
   * @private
   */
  reset() {
    this.imports.clear()
    this.exports.clear()
    this.components.clear()
  }

  /**
   * Generate automatic runtime import
   * @private
   */
  generateAutomaticRuntimeImport() {
    const source = this.options.jsxImportSource

    if (this.options.development) {
      return `import { jsxDEV as _jsxDEV } from '${source}/jsx-dev-runtime'`
    }

    return `import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from '${source}/jsx-runtime'`
  }

  /**
   * Collect import statement
   * @private
   */
  collectImport(importCode) {
    this.imports.add(importCode.trim())
  }

  /**
   * Collect export statement
   * @private
   */
  collectExport(exportCode) {
    try {
      const exportInfo = this.jsxParser.parseExport(exportCode)

      if (exportInfo.type === 'default') {
        this.exports.set('default', exportCode)
      } else {
        // Store named exports
        this.exports.set(exportCode, exportInfo)
      }
    } catch (_error) {
      // Store as-is if parsing fails
      this.exports.set(exportCode, exportCode)
    }
  }

  /**
   * Collect JSX components used
   * @private
   */
  collectJsxComponents(jsxCode) {
    // Extract component names from JSX
    const componentPattern = /<([A-Z][A-Za-z0-9.]*)/g
    let match = componentPattern.exec(jsxCode)

    while (match !== null) {
      const componentName = match[1].split('.')[0]
      this.components.add(componentName)
      match = componentPattern.exec(jsxCode)
    }
  }

  /**
   * Process frontmatter
   * @private
   */
  processFrontmatter(frontmatter) {
    // Export frontmatter as metadata
    this.exports.set('frontmatter', `export const frontmatter = ${JSON.stringify(frontmatter)}`)
  }

  /**
   * Wrap JSX element
   * @private
   */
  wrapJsx(jsxCode) {
    if (this.options.jsxRuntime === 'automatic') {
      // For automatic runtime, JSX is used directly
      return jsxCode
    }

    // For classic runtime, we need to ensure React is imported
    if (!Array.from(this.imports).some((imp) => imp.includes('react'))) {
      this.imports.add("import React from 'react'")
    }

    return jsxCode
  }

  /**
   * Wrap JavaScript expression
   * @private
   */
  wrapExpression(expression) {
    return `{${expression}}`
  }

  /**
   * Wrap markdown content
   * @private
   */
  wrapMarkdown(markdown) {
    // Convert markdown to JSX-compatible string
    // This is simplified - full implementation would parse and convert
    const escaped = markdown.replace(/\\/g, '\\\\').replace(/'/g, "\\'").replace(/\n/g, '\\n')

    return `<MDXMarkdown>${escaped}</MDXMarkdown>`
  }

  /**
   * Generate imports section
   * @private
   */
  generateImports() {
    const imports = Array.from(this.imports)

    // Add MDX provider import if needed
    if (this.options.providerImportSource) {
      imports.unshift(`import { MDXProvider } from '${this.options.providerImportSource}'`)
    }

    // Add default imports for components if not already imported
    for (const component of this.components) {
      const hasImport = imports.some((imp) => imp.includes(component) || imp.includes('* as'))

      if (!hasImport && !this.isBuiltinComponent(component)) {
        // Component needs to be provided via context or props
        // Add to the expected components list
      }
    }

    return imports.join('\n')
  }

  /**
   * Generate component function
   * @private
   */
  generateComponent(bodyParts, metadata) {
    const componentName = metadata.componentName || 'MDXContent'

    const code = []

    code.push(`function ${componentName}(props) {`)
    code.push('  const { components, ...rest } = props')

    // Merge components from provider
    if (this.options.providerImportSource) {
      code.push('  const contextComponents = useMDXComponents(components)')
    } else {
      code.push('  const contextComponents = components || {}')
    }

    // Generate component body
    code.push('')
    code.push('  return (')

    if (bodyParts.length === 0) {
      code.push('    null')
    } else if (bodyParts.length === 1) {
      code.push(`    ${bodyParts[0]}`)
    } else {
      code.push('    <>')
      for (const part of bodyParts) {
        code.push(`      ${part}`)
      }
      code.push('    </>')
    }

    code.push('  )')
    code.push('}')

    return code.join('\n')
  }

  /**
   * Generate exports section
   * @private
   */
  generateExports() {
    const exports = []

    for (const [key, value] of this.exports) {
      if (key === 'default') {
        // Skip default export, handled separately
        continue
      }

      if (typeof value === 'string') {
        exports.push(value)
      } else {
        // Generate export from parsed info
        exports.push(this.generateExportStatement(value))
      }
    }

    return exports.join('\n')
  }

  /**
   * Generate export statement
   * @private
   */
  generateExportStatement(exportInfo) {
    if (exportInfo.type === 'named' && exportInfo.declaration) {
      return `export ${exportInfo.declaration}`
    }

    if (exportInfo.type === 'all' && exportInfo.source) {
      return `export * from '${exportInfo.source.value}'`
    }

    return ''
  }

  /**
   * Generate default export
   * @private
   */
  generateDefaultExport(metadata) {
    const componentName = metadata.componentName || 'MDXContent'

    // Check if user provided a default export
    if (this.exports.has('default')) {
      return this.exports.get('default')
    }

    // Generate default export for the component
    const exports = [`export default ${componentName}`]

    // Add component metadata
    exports.push(`${componentName}.isMDXComponent = true`)

    return exports.join('\n')
  }

  /**
   * Check if component is built-in
   * @private
   */
  isBuiltinComponent(name) {
    const builtins = [
      'Fragment',
      'MDXProvider',
      'MDXMarkdown',
      // HTML elements are lowercase, so uppercase names are components
    ]

    return builtins.includes(name) || /^[a-z]/.test(name)
  }

  /**
   * Generate development helpers
   * @private
   */
  generateDevHelpers(metadata) {
    if (!this.options.development) {
      return ''
    }

    const helpers = []

    // Add source location info
    if (metadata.filename) {
      helpers.push(`const __MDX_FILE = '${metadata.filename}'`)
    }

    // Add hot reload support
    if (metadata.hmr) {
      helpers.push(`
if (import.meta.hot) {
  import.meta.hot.accept()
}`)
    }

    return helpers.join('\n')
  }
}

/**
 * Create a code generator instance
 * @param {CodegenOptions} [options] - Generator options
 * @returns {CodeGenerator} Generator instance
 */
export function createCodeGenerator(options) {
  return new CodeGenerator(options)
}

/**
 * Quick generate ES module
 * @param {Array} tokens - MDX tokens
 * @param {Object} [options] - Generator options
 * @returns {string} Generated code
 */
export function generateCode(tokens, options = {}) {
  const generator = createCodeGenerator(options)
  return generator.generate(tokens)
}
