import { describe, expect, it } from 'vitest'
import { VERSION } from './index'

describe('faster-md-vite', () => {
  it('should export VERSION', () => {
    expect(VERSION).toBe('0.1.0')
  })
})
