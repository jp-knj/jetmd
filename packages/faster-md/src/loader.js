// WASM loader for faster-md
// Handles initialization and caching of the WASM module

let _wasmModule = null
let wasmInstance = null
let initPromise = null

/**
 * Initialize the WASM module
 * @returns {Promise<void>}
 */
async function initWasm() {
  if (wasmInstance) {
    return wasmInstance
  }

  if (initPromise) {
    return initPromise
  }

  initPromise = loadWasmModule()
  wasmInstance = await initPromise
  return wasmInstance
}

/**
 * Load the WASM module from file or URL
 * @returns {Promise<object>}
 */
async function loadWasmModule() {
  try {
    // In Node.js environment
    if (typeof process !== 'undefined' && process.versions && process.versions.node) {
      const { readFile } = await import('node:fs/promises')
      const { join } = await import('node:path')
      const { fileURLToPath } = await import('node:url')
      const { dirname } = await import('node:path')

      // Get the directory of this file
      const __filename = fileURLToPath(import.meta.url)
      const __dirname = dirname(__filename)

      // Load WASM file
      const wasmPath = join(__dirname, '..', '..', 'wasm', 'fmd_wasm_bg.wasm')
      const wasmBuffer = await readFile(wasmPath)

      // Import the JS bindings
      const wasm = await import('../../../crates/fmd-wasm/pkg/fmd_wasm.js')

      // Initialize with the buffer
      await wasm.default(wasmBuffer)

      return wasm
    }

    // In browser environment

    // Dynamic import for browser
    const wasm = await import('../wasm/fmd_wasm.js')
    await wasm.default()
    return wasm
  } catch (error) {
    throw new Error(`Failed to load WASM module: ${error.message}`)
  }
}

/**
 * Get the initialized WASM instance
 * @returns {Promise<object>}
 */
export async function getWasmInstance() {
  if (!wasmInstance) {
    await initWasm()
  }
  return wasmInstance
}

/**
 * Reset the WASM instance (useful for testing)
 */
export function resetWasm() {
  _wasmModule = null
  wasmInstance = null
  initPromise = null
}

/**
 * Check if WASM is initialized
 * @returns {boolean}
 */
export function isWasmInitialized() {
  return wasmInstance !== null
}
