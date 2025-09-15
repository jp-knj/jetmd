// Vite plugin for MDX
// Transforms .mdx files into ES modules

import { readFile } from 'node:fs/promises'
import { createFilter } from '@rollup/pluginutils'
import { compileMdx } from 'mdx-compiler'
import { SourceMapGenerator } from 'source-map'

/**
 * MDX Vite plugin options
 * @typedef {Object} MdxViteOptions
 * @property {string[]} [include] - Files to include (default: ['**/*.mdx'])
 * @property {string[]} [exclude] - Files to exclude (default: ['**/node_modules/**'])
 * @property {string} [jsxRuntime] - JSX runtime: 'classic' or 'automatic' (default: 'automatic')
 * @property {string} [jsxImportSource] - JSX import source (default: 'react')
 * @property {string} [pragma] - JSX pragma for classic runtime (default: 'React.createElement')
 * @property {string} [pragmaFrag] - Fragment pragma for classic runtime (default: 'React.Fragment')
 * @property {boolean} [development] - Development mode (default: false)
 * @property {string} [providerImportSource] - MDX provider import source
 * @property {boolean} [hmr] - Enable HMR (default: true)
 * @property {Function[]} [remarkPlugins] - Remark plugins (default: [])
 * @property {Function[]} [rehypePlugins] - Rehype plugins (default: [])
 * @property {Function[]} [recmaPlugins] - Recma plugins (default: [])
 * @property {Function} [beforeCompile] - Hook before compilation
 * @property {Function} [afterCompile] - Hook after compilation
 */

/**
 * Create Vite plugin for MDX processing
 * @param {MdxViteOptions} options - Plugin options
 * @returns {import('vite').Plugin} Vite plugin
 */
export function mdxPlugin(options = {}) {
  const {
    include = ['**/*.mdx'],
    exclude = ['**/node_modules/**'],
    jsxRuntime = 'automatic',
    jsxImportSource = 'react',
    pragma = 'React.createElement',
    pragmaFrag = 'React.Fragment',
    development = process.env.NODE_ENV === 'development',
    providerImportSource,
    hmr = true,
    remarkPlugins = [],
    rehypePlugins = [],
    recmaPlugins = [],
    beforeCompile,
    afterCompile,
  } = options

  const filter = createFilter(include, exclude)

  return {
    name: 'vite-plugin-mdx',
    
    enforce: 'pre', // Run before other transforms
    
    async transform(code, id) {
      if (!filter(id)) {
        return null
      }

      try {
        // Hook before compilation
        let mdxContent = code
        if (beforeCompile) {
          mdxContent = await beforeCompile(mdxContent, id)
        }

        // Compile MDX to JavaScript
        const compiled = await compileMdx(mdxContent, {
          filepath: id,
          jsxRuntime,
          jsxImportSource,
          pragma,
          pragmaFrag,
          development,
          providerImportSource,
          remarkPlugins,
          rehypePlugins,
          recmaPlugins,
          sourceMap: true,
        })

        // Hook after compilation
        let result = compiled
        if (afterCompile) {
          result = await afterCompile(result, id)
        }

        // Add HMR support
        const hmrCode = hmr ? generateHmrCode(id) : ''
        const finalCode = `${result.code}\n${hmrCode}`

        return {
          code: finalCode,
          map: result.map || null,
        }
      } catch (error) {
        // Enhanced error reporting
        const err = new Error(`MDX compilation failed for ${id}:\n${error.message}`)
        err.stack = error.stack
        err.loc = error.position || error.location
        throw err
      }
    },

    async handleHotUpdate({ file, server, modules }) {
      if (!hmr || !filter(file)) {
        return
      }

      // Custom HMR handling for MDX files
      const module = modules.find(m => filter(m.id || m.file))
      
      if (module) {
        // Invalidate the module
        server.moduleGraph.invalidateModule(module)
        
        // Send custom HMR event
        server.ws.send({
          type: 'custom',
          event: 'mdx-update',
          data: {
            file: module.file,
            timestamp: Date.now(),
          },
        })

        // Return the module to trigger Vite's HMR
        return [module]
      }
    },

    configureServer(server) {
      // Add custom middleware for MDX error overlay
      server.middlewares.use((req, res, next) => {
        if (req.url?.includes('/__mdx-error')) {
          res.setHeader('Content-Type', 'application/json')
          res.end(JSON.stringify({ error: 'MDX compilation error' }))
          return
        }
        next()
      })

      // Watch for MDX file changes
      server.watcher.add(include)
    },

    config(config) {
      // Ensure .mdx files are treated as modules
      return {
        resolve: {
          extensions: [...(config.resolve?.extensions || []), '.mdx'],
        },
        esbuild: {
          jsx: jsxRuntime === 'automatic' ? 'automatic' : 'transform',
          jsxFactory: pragma,
          jsxFragment: pragmaFrag,
          jsxImportSource: jsxRuntime === 'automatic' ? jsxImportSource : undefined,
        },
      }
    },
  }
}

/**
 * Generate HMR code for MDX modules
 */
function generateHmrCode(id) {
  return `
if (import.meta.hot) {
  import.meta.hot.accept()
  
  // Preserve component state during HMR
  const prevRefreshReg = window.$RefreshReg$
  const prevRefreshSig = window.$RefreshSig$
  
  import.meta.hot.dispose(() => {
    window.$RefreshReg$ = prevRefreshReg
    window.$RefreshSig$ = prevRefreshSig
  })
  
  // Custom MDX HMR handler
  import.meta.hot.on('mdx-update', (data) => {
    if (data.file === ${JSON.stringify(id)}) {
      console.log('[MDX HMR] Updating:', data.file)
      import.meta.hot.invalidate()
    }
  })
}`.trim()
}

/**
 * Create source map from MDX to original file
 */
function createSourceMap(filename, source, generated) {
  const generator = new SourceMapGenerator({
    file: filename,
    sourceRoot: '',
  })

  // Simple line-by-line mapping
  const sourceLines = source.split('\n')
  const generatedLines = generated.split('\n')

  for (let i = 0; i < Math.min(sourceLines.length, generatedLines.length); i++) {
    generator.addMapping({
      source: filename,
      original: { line: i + 1, column: 0 },
      generated: { line: i + 1, column: 0 },
    })
  }

  generator.setSourceContent(filename, source)
  return generator.toString()
}

/**
 * Preset configurations for popular frameworks
 */
export const presets = {
  react: {
    jsxRuntime: 'automatic',
    jsxImportSource: 'react',
    providerImportSource: '@mdx-js/react',
  },
  
  preact: {
    jsxRuntime: 'automatic',
    jsxImportSource: 'preact',
    providerImportSource: '@mdx-js/preact',
  },
  
  vue: {
    jsxRuntime: 'automatic',
    jsxImportSource: 'vue',
    pragma: 'h',
    pragmaFrag: 'Fragment',
  },
  
  solid: {
    jsxRuntime: 'automatic',
    jsxImportSource: 'solid-js',
    providerImportSource: 'solid-mdx',
  },
}

/**
 * Helper to create plugin with preset
 */
export function mdxPluginWithPreset(preset, options = {}) {
  const presetConfig = typeof preset === 'string' ? presets[preset] : preset
  
  if (!presetConfig) {
    throw new Error(`Unknown MDX preset: ${preset}`)
  }
  
  return mdxPlugin({
    ...presetConfig,
    ...options,
  })
}

// Export utilities
export { createFilter }

// Default export
export default mdxPlugin