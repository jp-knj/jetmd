// Plugin registry for faster-md
// Manages and coordinates plugins for the markdown processor

/**
 * Plugin type definitions
 * @typedef {Object} Plugin
 * @property {string} name - Plugin name
 * @property {Function} [parser] - Parser transformer
 * @property {Function} [compiler] - Compiler transformer  
 * @property {Function} [transformer] - AST transformer
 * @property {Object} [options] - Plugin options
 * @property {string} [version] - Plugin version
 * @property {string[]} [dependencies] - Required plugins
 * @property {string[]} [conflicts] - Conflicting plugins
 */

/**
 * Plugin registry for managing plugins
 */
export class PluginRegistry {
  constructor() {
    this.plugins = new Map()
    this.order = []
    this.frozen = false
  }

  /**
   * Register a plugin
   * @param {Plugin} plugin - Plugin to register
   * @returns {PluginRegistry} This registry for chaining
   */
  register(plugin) {
    if (this.frozen) {
      throw new Error('Cannot register plugins on a frozen registry')
    }

    if (!plugin || typeof plugin !== 'object') {
      throw new TypeError('Plugin must be an object')
    }

    if (!plugin.name) {
      throw new Error('Plugin must have a name')
    }

    // Check for conflicts
    if (plugin.conflicts) {
      for (const conflict of plugin.conflicts) {
        if (this.plugins.has(conflict)) {
          throw new Error(
            `Plugin "${plugin.name}" conflicts with already registered plugin "${conflict}"`
          )
        }
      }
    }

    // Check dependencies
    if (plugin.dependencies) {
      for (const dep of plugin.dependencies) {
        if (!this.plugins.has(dep)) {
          throw new Error(
            `Plugin "${plugin.name}" requires plugin "${dep}" which is not registered`
          )
        }
      }
    }

    // Register the plugin
    this.plugins.set(plugin.name, plugin)
    this.order.push(plugin.name)

    return this
  }

  /**
   * Unregister a plugin
   * @param {string} name - Plugin name
   * @returns {PluginRegistry} This registry for chaining
   */
  unregister(name) {
    if (this.frozen) {
      throw new Error('Cannot unregister plugins from a frozen registry')
    }

    // Check if other plugins depend on this one
    for (const [pluginName, plugin] of this.plugins) {
      if (plugin.dependencies?.includes(name)) {
        throw new Error(
          `Cannot unregister plugin "${name}" because plugin "${pluginName}" depends on it`
        )
      }
    }

    this.plugins.delete(name)
    this.order = this.order.filter(n => n !== name)

    return this
  }

  /**
   * Get a plugin by name
   * @param {string} name - Plugin name
   * @returns {Plugin|undefined} Plugin or undefined
   */
  get(name) {
    return this.plugins.get(name)
  }

  /**
   * Check if a plugin is registered
   * @param {string} name - Plugin name
   * @returns {boolean} True if registered
   */
  has(name) {
    return this.plugins.has(name)
  }

  /**
   * Get all plugins in order
   * @returns {Plugin[]} Array of plugins
   */
  getAll() {
    return this.order.map(name => this.plugins.get(name))
  }

  /**
   * Get plugins by phase
   * @param {'parser'|'transformer'|'compiler'} phase - Processing phase
   * @returns {Plugin[]} Plugins that have the specified phase
   */
  getByPhase(phase) {
    return this.getAll().filter(plugin => typeof plugin[phase] === 'function')
  }

  /**
   * Apply parser plugins
   * @param {string} markdown - Markdown content
   * @param {Object} processor - Processor instance
   * @returns {Promise<string>} Processed markdown
   */
  async applyParsers(markdown, processor) {
    let result = markdown
    
    for (const plugin of this.getByPhase('parser')) {
      result = await plugin.parser(result, processor)
    }
    
    return result
  }

  /**
   * Apply transformer plugins
   * @param {Object} ast - AST
   * @param {Object} processor - Processor instance
   * @returns {Promise<Object>} Transformed AST
   */
  async applyTransformers(ast, processor) {
    let result = ast
    
    for (const plugin of this.getByPhase('transformer')) {
      result = await plugin.transformer(result, processor)
    }
    
    return result
  }

  /**
   * Apply compiler plugins
   * @param {Object} ast - AST
   * @param {Object} processor - Processor instance
   * @returns {Promise<Object>} Compiled result
   */
  async applyCompilers(ast, processor) {
    let result = ast
    
    for (const plugin of this.getByPhase('compiler')) {
      result = await plugin.compiler(result, processor)
    }
    
    return result
  }

  /**
   * Freeze the registry (prevent modifications)
   * @returns {PluginRegistry} This registry for chaining
   */
  freeze() {
    this.frozen = true
    return this
  }

  /**
   * Clone the registry
   * @returns {PluginRegistry} New registry with same plugins
   */
  clone() {
    const registry = new PluginRegistry()
    
    for (const plugin of this.getAll()) {
      registry.register(plugin)
    }
    
    return registry
  }

  /**
   * Clear all plugins
   * @returns {PluginRegistry} This registry for chaining
   */
  clear() {
    if (this.frozen) {
      throw new Error('Cannot clear a frozen registry')
    }

    this.plugins.clear()
    this.order = []
    
    return this
  }

  /**
   * Get registry info
   * @returns {Object} Registry information
   */
  info() {
    return {
      count: this.plugins.size,
      plugins: this.order,
      frozen: this.frozen,
      phases: {
        parser: this.getByPhase('parser').length,
        transformer: this.getByPhase('transformer').length,
        compiler: this.getByPhase('compiler').length,
      },
    }
  }
}

/**
 * Create a new plugin registry
 * @returns {PluginRegistry} New registry
 */
export function createRegistry() {
  return new PluginRegistry()
}

/**
 * Create a preset registry with common plugins
 * @param {'minimal'|'standard'|'full'} preset - Preset name
 * @returns {PluginRegistry} Registry with preset plugins
 */
export function createPresetRegistry(preset = 'standard') {
  const registry = createRegistry()

  switch (preset) {
    case 'minimal':
      // Just basic markdown
      break

    case 'standard':
      // GFM and frontmatter
      // These will be imported when we create the actual plugin files
      break

    case 'full':
      // Everything including syntax highlighting
      break

    default:
      throw new Error(`Unknown preset: ${preset}`)
  }

  return registry
}

/**
 * Plugin builder helper
 */
export class PluginBuilder {
  constructor(name) {
    this.plugin = { name }
  }

  parser(fn) {
    this.plugin.parser = fn
    return this
  }

  transformer(fn) {
    this.plugin.transformer = fn
    return this
  }

  compiler(fn) {
    this.plugin.compiler = fn
    return this
  }

  options(opts) {
    this.plugin.options = opts
    return this
  }

  version(v) {
    this.plugin.version = v
    return this
  }

  dependencies(...deps) {
    this.plugin.dependencies = deps
    return this
  }

  conflicts(...names) {
    this.plugin.conflicts = names
    return this
  }

  build() {
    return this.plugin
  }
}

/**
 * Create a plugin using the builder
 * @param {string} name - Plugin name
 * @returns {PluginBuilder} Plugin builder
 */
export function createPlugin(name) {
  return new PluginBuilder(name)
}

// Export default registry instance
export const defaultRegistry = createRegistry()