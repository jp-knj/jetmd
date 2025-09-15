// JSX parser integration for MDX
// Uses acorn and acorn-jsx to parse JavaScript/JSX expressions

import * as acorn from 'acorn'
import jsx from 'acorn-jsx'

// Create parser with JSX support
const Parser = acorn.Parser.extend(jsx())

/**
 * JSX parsing options
 * @typedef {Object} JsxOptions
 * @property {string} [ecmaVersion='latest'] - ECMAScript version
 * @property {string} [sourceType='module'] - Source type
 * @property {boolean} [ranges=true] - Include ranges
 * @property {boolean} [locations=true] - Include locations
 */

/**
 * JSX parser for MDX
 */
export class JsxParser {
  constructor(options = {}) {
    this.options = {
      ecmaVersion: 'latest',
      sourceType: 'module',
      ranges: true,
      locations: true,
      ...options,
    }
  }

  /**
   * Parse JSX element
   * @param {string} jsxCode - JSX code
   * @returns {Object} Parsed AST
   */
  parseElement(jsxCode) {
    try {
      // Wrap in expression to parse as JSX
      const wrapped = `(${jsxCode})`
      const ast = Parser.parse(wrapped, this.options)

      // Extract the JSX element from the expression
      if (ast.body?.[0]?.expression) {
        return ast.body[0].expression
      }

      throw new Error('Failed to parse JSX element')
    } catch (error) {
      throw new Error(`JSX parse error: ${error.message}`)
    }
  }

  /**
   * Parse JSX expression
   * @param {string} expression - JavaScript expression
   * @returns {Object} Parsed AST
   */
  parseExpression(expression) {
    try {
      // Parse as an expression
      const ast = Parser.parseExpressionAt(expression, 0, this.options)
      return ast
    } catch (error) {
      // Try parsing as a statement
      try {
        const ast = Parser.parse(expression, this.options)
        if (ast.body && ast.body.length > 0) {
          return ast.body[0]
        }
      } catch (_statementError) {
        // Ignore statement error, throw original
      }

      throw new Error(`Expression parse error: ${error.message}`)
    }
  }

  /**
   * Parse import statement
   * @param {string} importCode - Import statement
   * @returns {Object} Parsed import info
   */
  parseImport(importCode) {
    try {
      const ast = Parser.parse(importCode, this.options)

      if (!ast.body || ast.body.length === 0) {
        throw new Error('No import statement found')
      }

      const importNode = ast.body[0]
      if (importNode.type !== 'ImportDeclaration') {
        throw new Error('Not an import statement')
      }

      return this.extractImportInfo(importNode)
    } catch (error) {
      throw new Error(`Import parse error: ${error.message}`)
    }
  }

  /**
   * Parse export statement
   * @param {string} exportCode - Export statement
   * @returns {Object} Parsed export info
   */
  parseExport(exportCode) {
    try {
      const ast = Parser.parse(exportCode, this.options)

      if (!ast.body || ast.body.length === 0) {
        throw new Error('No export statement found')
      }

      const exportNode = ast.body[0]

      // Handle different export types
      if (exportNode.type === 'ExportDefaultDeclaration') {
        return {
          type: 'default',
          declaration: exportNode.declaration,
        }
      }

      if (exportNode.type === 'ExportNamedDeclaration') {
        return {
          type: 'named',
          declaration: exportNode.declaration,
          specifiers: exportNode.specifiers,
          source: exportNode.source,
        }
      }

      if (exportNode.type === 'ExportAllDeclaration') {
        return {
          type: 'all',
          source: exportNode.source,
        }
      }

      throw new Error('Unknown export type')
    } catch (error) {
      throw new Error(`Export parse error: ${error.message}`)
    }
  }

  /**
   * Extract import information
   * @private
   */
  extractImportInfo(node) {
    const info = {
      source: node.source.value,
      specifiers: [],
    }

    for (const specifier of node.specifiers) {
      if (specifier.type === 'ImportDefaultSpecifier') {
        info.specifiers.push({
          type: 'default',
          local: specifier.local.name,
        })
      } else if (specifier.type === 'ImportSpecifier') {
        info.specifiers.push({
          type: 'named',
          imported: specifier.imported.name,
          local: specifier.local.name,
        })
      } else if (specifier.type === 'ImportNamespaceSpecifier') {
        info.specifiers.push({
          type: 'namespace',
          local: specifier.local.name,
        })
      }
    }

    return info
  }

  /**
   * Validate JSX syntax
   * @param {string} jsxCode - JSX code to validate
   * @returns {Object} Validation result
   */
  validate(jsxCode) {
    try {
      this.parseElement(jsxCode)
      return { valid: true }
    } catch (error) {
      return {
        valid: false,
        error: error.message,
        location: error.loc,
      }
    }
  }

  /**
   * Transform JSX to function calls
   * @param {Object} jsxAst - JSX AST node
   * @param {Object} [options] - Transform options
   * @returns {Object} Transformed AST
   */
  transformJsx(jsxAst, options = {}) {
    const pragma = options.pragma || 'React.createElement'
    const pragmaFrag = options.pragmaFrag || 'React.Fragment'

    // This is a simplified version - full implementation would
    // recursively transform the entire AST
    return this.transformNode(jsxAst, pragma, pragmaFrag)
  }

  /**
   * Transform a single node
   * @private
   */
  transformNode(node, pragma, pragmaFrag) {
    if (!node || typeof node !== 'object') {
      return node
    }

    // Handle JSX elements
    if (node.type === 'JSXElement') {
      return this.transformElement(node, pragma, pragmaFrag)
    }

    // Handle JSX fragments
    if (node.type === 'JSXFragment') {
      return this.transformFragment(node, pragma, pragmaFrag)
    }

    // Handle JSX expressions
    if (node.type === 'JSXExpressionContainer') {
      return node.expression
    }

    // Handle JSX text
    if (node.type === 'JSXText') {
      const value = node.value.trim()
      return value ? { type: 'Literal', value } : null
    }

    return node
  }

  /**
   * Transform JSX element to function call
   * @private
   */
  transformElement(element, pragma, pragmaFrag) {
    const tagName = this.getElementName(element.openingElement)

    // Create pragma call
    return {
      type: 'CallExpression',
      callee: this.parsePragma(pragma),
      arguments: [
        // Component or tag name
        /^[A-Z]/.test(tagName)
          ? { type: 'Identifier', name: tagName }
          : { type: 'Literal', value: tagName },
        // Props
        this.transformProps(element.openingElement.attributes),
        // Children
        ...this.transformChildren(element.children, pragma, pragmaFrag),
      ],
    }
  }

  /**
   * Transform JSX fragment
   * @private
   */
  transformFragment(fragment, pragma, pragmaFrag) {
    return {
      type: 'CallExpression',
      callee: this.parsePragma(pragma),
      arguments: [
        this.parsePragma(pragmaFrag),
        null,
        ...this.transformChildren(fragment.children, pragma, pragmaFrag),
      ],
    }
  }

  /**
   * Get element name
   * @private
   */
  getElementName(openingElement) {
    const name = openingElement.name

    if (name.type === 'JSXIdentifier') {
      return name.name
    }

    if (name.type === 'JSXMemberExpression') {
      return this.getMemberExpressionName(name)
    }

    return 'unknown'
  }

  /**
   * Get member expression name
   * @private
   */
  getMemberExpressionName(node) {
    if (node.type === 'JSXIdentifier') {
      return node.name
    }

    if (node.type === 'JSXMemberExpression') {
      return `${this.getMemberExpressionName(node.object)}.${node.property.name}`
    }

    return 'unknown'
  }

  /**
   * Transform props
   * @private
   */
  transformProps(attributes) {
    if (!attributes || attributes.length === 0) {
      return { type: 'Literal', value: null }
    }

    const props = []

    for (const attr of attributes) {
      if (attr.type === 'JSXAttribute') {
        props.push({
          type: 'Property',
          key: { type: 'Identifier', name: attr.name.name },
          value: attr.value ? this.transformNode(attr.value) : { type: 'Literal', value: true },
        })
      } else if (attr.type === 'JSXSpreadAttribute') {
        // Handle spread props
        props.push({
          type: 'SpreadElement',
          argument: attr.argument,
        })
      }
    }

    return {
      type: 'ObjectExpression',
      properties: props,
    }
  }

  /**
   * Transform children
   * @private
   */
  transformChildren(children, pragma, pragmaFrag) {
    if (!children || children.length === 0) {
      return []
    }

    return children
      .map((child) => this.transformNode(child, pragma, pragmaFrag))
      .filter((child) => child !== null)
  }

  /**
   * Parse pragma string
   * @private
   */
  parsePragma(pragma) {
    const parts = pragma.split('.')

    if (parts.length === 1) {
      return { type: 'Identifier', name: parts[0] }
    }

    // Build member expression
    let expr = { type: 'Identifier', name: parts[0] }

    for (let i = 1; i < parts.length; i++) {
      expr = {
        type: 'MemberExpression',
        object: expr,
        property: { type: 'Identifier', name: parts[i] },
      }
    }

    return expr
  }
}

/**
 * Create a JSX parser instance
 * @param {JsxOptions} [options] - Parser options
 * @returns {JsxParser} Parser instance
 */
export function createJsxParser(options) {
  return new JsxParser(options)
}

/**
 * Quick parse JSX
 * @param {string} jsxCode - JSX code
 * @returns {Object} Parsed AST
 */
export function parseJsx(jsxCode) {
  const parser = createJsxParser()
  return parser.parseElement(jsxCode)
}
