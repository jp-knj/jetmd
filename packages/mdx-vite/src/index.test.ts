import { describe, expect, it } from 'vitest'
import { VERSION } from './index'

describe('mdx-vite', () => {
  it('should export VERSION', () => {
    expect(VERSION).toBe('0.1.0')
  })
})
