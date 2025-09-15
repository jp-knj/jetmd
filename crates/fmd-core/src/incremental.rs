// Incremental parsing cache
use crate::{Node, NodeType};
use std::collections::HashMap;

/// Cache for incremental parsing
#[derive(Debug, Clone)]
pub struct IncrementalCache {
    /// Cached nodes by content hash
    node_cache: HashMap<u64, Node>,
    /// Line-level caches for quick lookups
    line_cache: HashMap<usize, Vec<Node>>,
    /// Statistics
    hits: usize,
    misses: usize,
}

impl IncrementalCache {
    pub fn new() -> Self {
        Self {
            node_cache: HashMap::new(),
            line_cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    
    /// Get the cache size
    pub fn size(&self) -> usize {
        self.node_cache.len()
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.node_cache.clear();
        self.line_cache.clear();
        self.hits = 0;
        self.misses = 0;
    }
    
    /// Try to get a cached node
    pub fn get(&mut self, hash: u64) -> Option<&Node> {
        if let Some(node) = self.node_cache.get(&hash) {
            self.hits += 1;
            Some(node)
        } else {
            self.misses += 1;
            None
        }
    }
    
    /// Store a node in the cache
    pub fn put(&mut self, hash: u64, node: Node) {
        self.node_cache.insert(hash, node);
    }
    
    /// Get cache hit rate
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    
    /// Cache nodes by line for line-level reuse
    pub fn cache_line(&mut self, line: usize, nodes: Vec<Node>) {
        self.line_cache.insert(line, nodes);
    }
    
    /// Get cached nodes for a line
    pub fn get_line(&self, line: usize) -> Option<&Vec<Node>> {
        self.line_cache.get(&line)
    }
}

/// Calculate a hash for content (simple FNV-1a hash)
pub fn content_hash(content: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in content.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Diff result for incremental parsing
#[derive(Debug)]
pub struct DiffResult {
    pub unchanged: Vec<(usize, usize)>, // (start, end) line ranges
    pub changed: Vec<(usize, usize)>,   // (start, end) line ranges
    pub reuse_rate: f64,
}

/// Calculate diff between old and new content
pub fn calculate_diff(old: &str, new: &str) -> DiffResult {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();
    
    let mut unchanged = Vec::new();
    let mut changed = Vec::new();
    let mut i = 0;
    let mut j = 0;
    
    // Simple line matching algorithm
    while i < old_lines.len() && j < new_lines.len() {
        if old_lines[i] == new_lines[j] {
            // Found matching line, extend unchanged range
            let start = j;
            while i < old_lines.len() && j < new_lines.len() && old_lines[i] == new_lines[j] {
                i += 1;
                j += 1;
            }
            unchanged.push((start, j));
        } else {
            // Lines don't match, find next match
            let start = j;
            let mut found = false;
            
            // Look ahead for matching lines
            for look_ahead in 1..5 {
                if j + look_ahead < new_lines.len() {
                    for old_ahead in 0..5 {
                        if i + old_ahead < old_lines.len() 
                            && old_lines[i + old_ahead] == new_lines[j + look_ahead] {
                            changed.push((start, j + look_ahead));
                            i += old_ahead;
                            j += look_ahead;
                            found = true;
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }
            
            if !found {
                i += 1;
                j += 1;
                changed.push((start, j));
            }
        }
    }
    
    // Handle remaining lines
    if j < new_lines.len() {
        changed.push((j, new_lines.len()));
    }
    
    let total_lines = new_lines.len();
    let unchanged_lines: usize = unchanged.iter().map(|(s, e)| e - s).sum();
    let reuse_rate = if total_lines > 0 {
        unchanged_lines as f64 / total_lines as f64
    } else {
        0.0
    };
    
    DiffResult {
        unchanged,
        changed,
        reuse_rate,
    }
}