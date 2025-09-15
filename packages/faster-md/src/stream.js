// Stream support for faster-md
// Provides streaming markdown processing for Node.js

import { Transform } from 'node:stream'
import { createProcessor } from './processor.js'

/**
 * Stream options
 * @typedef {Object} StreamOptions
 * @property {number} [highWaterMark=16384] - Stream buffer size
 * @property {string} [encoding='utf8'] - Input encoding
 * @property {Object} [processorOptions] - Options for the processor
 * @property {boolean} [chunked=false] - Process in chunks (for large documents)
 * @property {string} [chunkDelimiter='\n\n'] - Delimiter for chunking
 */

/**
 * Markdown transform stream
 */
export class MarkdownStream extends Transform {
  constructor(options = {}) {
    const { highWaterMark = 16384, encoding = 'utf8', ...rest } = options

    super({
      highWaterMark,
      encoding,
      decodeStrings: false,
      objectMode: false,
    })

    this.processorOptions = rest.processorOptions || {}
    this.processor = createProcessor(this.processorOptions)
    this.chunked = rest.chunked || false
    this.chunkDelimiter = rest.chunkDelimiter || '\n\n'
    this.buffer = ''
  }

  /**
   * Transform implementation
   * @private
   */
  async _transform(chunk, encoding, callback) {
    try {
      // Convert chunk to string if needed
      const text = chunk.toString(encoding || 'utf8')

      if (this.chunked) {
        // Add to buffer
        this.buffer += text

        // Process complete chunks
        const chunks = this.buffer.split(this.chunkDelimiter)

        // Keep last incomplete chunk in buffer
        this.buffer = chunks.pop() || ''

        // Process complete chunks
        for (const chunkText of chunks) {
          if (chunkText.trim()) {
            const html = await this.processor.process(chunkText)
            this.push(html)
            this.push('\n') // Add separator between chunks
          }
        }
      } else {
        // Non-chunked mode: accumulate all input
        this.buffer += text
      }

      callback()
    } catch (error) {
      callback(error)
    }
  }

  /**
   * Flush implementation
   * @private
   */
  async _flush(callback) {
    try {
      if (this.buffer.trim()) {
        const html = await this.processor.process(this.buffer)
        this.push(html)
      }
      callback()
    } catch (error) {
      callback(error)
    }
  }
}

/**
 * Line-by-line markdown stream processor
 */
export class LineStream extends Transform {
  constructor(options = {}) {
    super({
      ...options,
      objectMode: true,
    })

    this.processor = createProcessor(options.processorOptions || {})
    this.lineBuffer = ''
  }

  /**
   * Transform implementation for line processing
   * @private
   */
  async _transform(chunk, _encoding, callback) {
    try {
      const text = chunk.toString()
      const lines = (this.lineBuffer + text).split('\n')

      // Keep last incomplete line
      this.lineBuffer = lines.pop() || ''

      // Process complete lines
      for (const line of lines) {
        // Detect block boundaries and process
        // This is simplified - real implementation would properly
        // detect markdown block boundaries
        if (line.trim() || this.currentBlock) {
          // Process line as markdown
          const html = await this.processor.process(line)
          this.push({ line, html })
        }
      }

      callback()
    } catch (error) {
      callback(error)
    }
  }

  /**
   * Flush remaining line
   * @private
   */
  async _flush(callback) {
    try {
      if (this.lineBuffer.trim()) {
        const html = await this.processor.process(this.lineBuffer)
        this.push({ line: this.lineBuffer, html })
      }
      callback()
    } catch (error) {
      callback(error)
    }
  }
}

/**
 * Create a markdown transform stream
 * @param {StreamOptions} [options] - Stream options
 * @returns {MarkdownStream} Transform stream
 */
export function createStream(options) {
  return new MarkdownStream(options)
}

/**
 * Create a line-by-line processor stream
 * @param {StreamOptions} [options] - Stream options
 * @returns {LineStream} Transform stream
 */
export function createLineStream(options) {
  return new LineStream(options)
}

/**
 * Process a readable stream of markdown
 * @param {ReadableStream} input - Input stream
 * @param {StreamOptions} [options] - Processing options
 * @returns {Promise<string>} Processed HTML
 */
export async function processStream(input, options = {}) {
  return new Promise((resolve, reject) => {
    const chunks = []
    const stream = createStream(options)

    stream.on('data', (chunk) => {
      chunks.push(chunk)
    })

    stream.on('end', () => {
      resolve(chunks.join(''))
    })

    stream.on('error', reject)

    input.pipe(stream)
  })
}

/**
 * Create a duplex stream for bidirectional processing
 */
export class DuplexMarkdownStream extends Transform {
  constructor(options = {}) {
    super({
      ...options,
      objectMode: true,
    })

    this.processor = createProcessor(options.processorOptions || {})
  }

  async _transform(chunk, _encoding, callback) {
    try {
      const input = typeof chunk === 'string' ? chunk : chunk.markdown
      const html = await this.processor.process(input)

      this.push({
        markdown: input,
        html,
        metadata: chunk.metadata || {},
      })

      callback()
    } catch (error) {
      callback(error)
    }
  }
}

/**
 * Create a duplex markdown stream
 * @param {StreamOptions} [options] - Stream options
 * @returns {DuplexMarkdownStream} Duplex stream
 */
export function createDuplexStream(options) {
  return new DuplexMarkdownStream(options)
}
