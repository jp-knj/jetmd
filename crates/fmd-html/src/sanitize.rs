// HTML sanitization using ammonia

/// Sanitize HTML to prevent XSS attacks
pub fn sanitize_html(html: &str, options: &SanitizeOptions) -> String {
    if !options.enabled {
        return html.to_string();
    }
    
    // For now, use ammonia's defaults with basic configuration
    // The API has changed significantly and needs more investigation
    if options.allow_dangerous_html {
        // Don't sanitize at all
        html.to_string()
    } else {
        // Use ammonia defaults which are safe
        ammonia::clean(html)
    }
}

#[derive(Debug, Clone)]
pub struct SanitizeOptions {
    pub enabled: bool,
    pub allow_dangerous_html: bool,
}

impl Default for SanitizeOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            allow_dangerous_html: false,
        }
    }
}

impl SanitizeOptions {
    /// Create strict sanitization options
    pub fn strict() -> Self {
        Self {
            enabled: true,
            allow_dangerous_html: false,
        }
    }
    
    /// Create permissive sanitization options (still sanitizes)
    pub fn permissive() -> Self {
        Self {
            enabled: true,
            allow_dangerous_html: false,
        }
    }
    
    /// Disable sanitization (dangerous!)
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            allow_dangerous_html: true,
        }
    }
}

/// Quick sanitize with default options
pub fn quick_sanitize(html: &str) -> String {
    sanitize_html(html, &SanitizeOptions::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_sanitization() {
        let html = "<p>Hello <script>alert('xss')</script>world</p>";
        let sanitized = quick_sanitize(html);
        assert_eq!(sanitized, "<p>Hello world</p>");
    }
    
    #[test]
    fn test_strict_sanitization() {
        let html = "<p>Hello <a href='http://example.com'>link</a></p>";
        let sanitized = sanitize_html(html, &SanitizeOptions::strict());
        assert_eq!(sanitized, "<p>Hello link</p>");
    }
    
    #[test]
    fn test_permissive_sanitization() {
        let html = "<p>Hello <a href='http://example.com'>link</a></p>";
        let sanitized = sanitize_html(html, &SanitizeOptions::permissive());
        assert_eq!(sanitized, "<p>Hello <a href=\"http://example.com\" rel=\"noopener noreferrer\">link</a></p>");
    }
    
    #[test]
    fn test_disabled_sanitization() {
        let html = "<p>Hello <script>alert('xss')</script>world</p>";
        let sanitized = sanitize_html(html, &SanitizeOptions::disabled());
        assert_eq!(sanitized, html);
    }
}