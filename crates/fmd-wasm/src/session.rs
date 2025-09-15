// WASM session management for incremental parsing
use wasm_bindgen::prelude::*;
use fmd_core::{Document, ProcessorOptions, Node, IncrementalCache};
use serde_wasm_bindgen::{from_value, to_value};
use std::collections::HashMap;

/// Session for incremental parsing
#[wasm_bindgen]
pub struct ParseSession {
    session_id: String,
    cache: IncrementalCache,
    last_content: String,
    last_ast: Option<Node>,
    parse_count: usize,
    total_reuse: usize,
}

#[wasm_bindgen]
impl ParseSession {
    /// Create a new parsing session
    #[wasm_bindgen(constructor)]
    pub fn new(session_id: Option<String>) -> Self {
        Self {
            session_id: session_id.unwrap_or_else(|| {
                format!("session_{}", js_sys::Date::now() as u64)
            }),
            cache: IncrementalCache::new(),
            last_content: String::new(),
            last_ast: None,
            parse_count: 0,
            total_reuse: 0,
        }
    }
    
    /// Get session ID
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.session_id.clone()
    }
    
    /// Parse incrementally with caching
    pub fn parse(&mut self, content: &str, options: JsValue) -> Result<JsValue, JsValue> {
        let opts: ProcessorOptions = if options.is_undefined() || options.is_null() {
            ProcessorOptions::default()
        } else {
            from_value(options)
                .map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
        };
        
        self.parse_count += 1;
        
        // Calculate diff if we have previous content
        let reuse_stats = if !self.last_content.is_empty() {
            calculate_diff(&self.last_content, content)
        } else {
            DiffStats::default()
        };
        
        // Parse with incremental cache
        let doc = Document::new(content);
        let result = if let Some(ref last_ast) = self.last_ast {
            fmd_core::parse_incremental(&doc, opts, last_ast, &mut self.cache)
        } else {
            fmd_core::parse(&doc, opts)
        };
        
        // Update session state
        if result.success {
            self.last_ast = Some(result.ast.clone());
            self.last_content = content.to_string();
            self.total_reuse += reuse_stats.unchanged_lines;
        }
        
        // Build response with session metadata
        let response = serde_json::json!({
            "ast": result.ast,
            "success": result.success,
            "errors": result.errors,
            "session": {
                "id": self.session_id,
                "parseCount": self.parse_count,
                "cacheSize": self.cache.size(),
                "reuseRate": if self.parse_count > 1 {
                    reuse_stats.unchanged_lines as f64 / reuse_stats.total_lines as f64
                } else {
                    0.0
                },
                "totalReuse": self.total_reuse,
            }
        });
        
        to_value(&response).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
    
    /// Clear the session cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.last_content.clear();
        self.last_ast = None;
        self.total_reuse = 0;
    }
    
    /// Get session statistics
    pub fn getStats(&self) -> Result<JsValue, JsValue> {
        let stats = serde_json::json!({
            "sessionId": self.session_id,
            "parseCount": self.parse_count,
            "cacheSize": self.cache.size(),
            "hasLastAst": self.last_ast.is_some(),
            "lastContentLength": self.last_content.len(),
            "totalReuse": self.total_reuse,
            "averageReuseRate": if self.parse_count > 0 {
                self.total_reuse as f64 / self.parse_count as f64
            } else {
                0.0
            }
        });
        
        to_value(&stats).map_err(|e| JsValue::from_str(&format!("Statistics error: {}", e)))
    }
}

/// Global session manager
#[wasm_bindgen]
pub struct SessionManager {
    sessions: HashMap<String, ParseSession>,
    max_sessions: usize,
}

#[wasm_bindgen]
impl SessionManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            max_sessions: 100,
        }
    }
    
    /// Create a new session
    pub fn createSession(&mut self, session_id: Option<String>) -> String {
        // Clean up old sessions if we're at the limit
        if self.sessions.len() >= self.max_sessions {
            self.cleanupOldSessions();
        }
        
        let session = ParseSession::new(session_id);
        let id = session.id();
        self.sessions.insert(id.clone(), session);
        id
    }
    
    /// Get an existing session
    pub fn getSession(&mut self, session_id: &str) -> Option<ParseSession> {
        self.sessions.remove(session_id)
    }
    
    /// Remove a session
    pub fn removeSession(&mut self, session_id: &str) -> bool {
        self.sessions.remove(session_id).is_some()
    }
    
    /// Get all session IDs
    pub fn listSessions(&self) -> Vec<JsValue> {
        self.sessions.keys()
            .map(|k| JsValue::from_str(k))
            .collect()
    }
    
    /// Clean up old sessions (keeps most recent half)
    fn cleanupOldSessions(&mut self) {
        let to_remove = self.sessions.len() / 2;
        let mut keys: Vec<_> = self.sessions.keys().cloned().collect();
        keys.truncate(to_remove);
        for key in keys {
            self.sessions.remove(&key);
        }
    }
}

#[derive(Default)]
struct DiffStats {
    total_lines: usize,
    unchanged_lines: usize,
    added_lines: usize,
    removed_lines: usize,
}

fn calculate_diff(old: &str, new: &str) -> DiffStats {
    let old_lines: Vec<_> = old.lines().collect();
    let new_lines: Vec<_> = new.lines().collect();
    
    let mut stats = DiffStats {
        total_lines: new_lines.len(),
        unchanged_lines: 0,
        added_lines: 0,
        removed_lines: 0,
    };
    
    // Simple line-by-line comparison (could be improved with LCS algorithm)
    let min_len = old_lines.len().min(new_lines.len());
    for i in 0..min_len {
        if old_lines[i] == new_lines[i] {
            stats.unchanged_lines += 1;
        }
    }
    
    if new_lines.len() > old_lines.len() {
        stats.added_lines = new_lines.len() - old_lines.len();
    } else {
        stats.removed_lines = old_lines.len() - new_lines.len();
    }
    
    stats
}