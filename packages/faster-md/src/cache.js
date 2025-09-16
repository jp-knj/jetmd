// Simple in-memory cache for markdown rendering
const cache = new Map();
const MAX_CACHE_SIZE = 100; // Maximum number of cached entries
const CACHE_TTL = 60 * 60 * 1000; // 1 hour in milliseconds

/**
 * Generate cache key from content and options
 */
function getCacheKey(content, options = {}) {
  // Simple hash function for cache key
  const optionsKey = JSON.stringify(options);
  const contentHash = simpleHash(content);
  return `${contentHash}:${optionsKey}`;
}

/**
 * Simple hash function (djb2 algorithm)
 */
function simpleHash(str) {
  let hash = 5381;
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i);
    hash = ((hash << 5) + hash) + char; // hash * 33 + char
  }
  return hash.toString(36);
}

/**
 * Get cached result
 */
export function getCached(content, options) {
  const key = getCacheKey(content, options);
  const entry = cache.get(key);
  
  if (entry) {
    // Check if cache entry is still valid
    if (Date.now() - entry.timestamp < CACHE_TTL) {
      entry.hits++;
      return entry.result;
    } else {
      // Expired entry
      cache.delete(key);
    }
  }
  
  return null;
}

/**
 * Set cache entry
 */
export function setCached(content, options, result) {
  const key = getCacheKey(content, options);
  
  // Limit cache size
  if (cache.size >= MAX_CACHE_SIZE) {
    // Remove least recently used entries
    const entries = Array.from(cache.entries());
    entries.sort((a, b) => a[1].lastAccess - b[1].lastAccess);
    
    // Remove oldest 10% of entries
    const toRemove = Math.floor(MAX_CACHE_SIZE * 0.1);
    for (let i = 0; i < toRemove; i++) {
      cache.delete(entries[i][0]);
    }
  }
  
  cache.set(key, {
    result,
    timestamp: Date.now(),
    lastAccess: Date.now(),
    hits: 0
  });
}

/**
 * Clear cache
 */
export function clearCache() {
  cache.clear();
}

/**
 * Get cache statistics
 */
export function getCacheStats() {
  let totalHits = 0;
  let totalSize = 0;
  
  for (const [key, entry] of cache.entries()) {
    totalHits += entry.hits;
    totalSize += JSON.stringify(entry.result).length;
  }
  
  return {
    entries: cache.size,
    totalHits,
    totalSizeKB: (totalSize / 1024).toFixed(2),
    maxSize: MAX_CACHE_SIZE
  };
}