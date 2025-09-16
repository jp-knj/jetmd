// Parser implementation using pulldown-cmark
use crate::{
    ast::{Node, NodeType},
    position::Position,
    Document, ProcessorOptions,
};
use pulldown_cmark::{
    CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, Options, Parser, Tag, TagEnd,
};

pub fn parse_with_pulldown(doc: &Document, options: ProcessorOptions) -> Node {
    let mut pulldown_options = Options::empty();

    // Map our options to pulldown-cmark options
    if options.gfm || options.gfm_options.tables {
        pulldown_options.insert(Options::ENABLE_TABLES);
    }
    if options.gfm || options.gfm_options.strikethrough {
        pulldown_options.insert(Options::ENABLE_STRIKETHROUGH);
    }
    if options.gfm || options.gfm_options.tasklists {
        pulldown_options.insert(Options::ENABLE_TASKLISTS);
    }
    if options.frontmatter {
        pulldown_options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    }
    if options.math {
        pulldown_options.insert(Options::ENABLE_MATH);
    }

    let parser = Parser::new_ext(&doc.content, pulldown_options);
    let mut builder = AstBuilder::new(options);

    for (event, range) in parser.into_offset_iter() {
        builder.handle_event(event, range, &doc.content);
    }

    builder.finish()
}

struct AstBuilder {
    root: Node,
    stack: Vec<Node>,
    list_stack: Vec<ListContext>,
    options: ProcessorOptions,
    content: Vec<String>,
}

struct ListContext {
    ordered: bool,
    start: Option<u64>,
    tight: bool,
}

impl AstBuilder {
    fn new(options: ProcessorOptions) -> Self {
        Self {
            root: Node {
                node_type: NodeType::Root,
                children: Vec::new(),
                ..Default::default()
            },
            stack: Vec::new(),
            list_stack: Vec::new(),
            options,
            content: Vec::new(),
        }
    }

    fn handle_event(&mut self, event: Event, range: std::ops::Range<usize>, source: &str) {
        let position = if self.options.position {
            Some(self.range_to_position(range.clone(), source))
        } else {
            None
        };

        match event {
            Event::Start(tag) => self.handle_start_tag(tag, position),
            Event::End(tag) => self.handle_end_tag(tag),
            Event::Text(text) => self.handle_text(text, position),
            Event::Code(code) => self.handle_inline_code(code, position),
            Event::Html(html) => self.handle_html(html, position),
            Event::SoftBreak => self.handle_soft_break(position),
            Event::HardBreak => self.handle_hard_break(position),
            Event::Rule => self.handle_rule(position),
            Event::TaskListMarker(checked) => self.handle_task_list_marker(checked),
            Event::InlineMath(math) => self.handle_inline_math(math, position),
            Event::DisplayMath(math) => self.handle_display_math(math, position),
            _ => {} // Ignore other events for now
        }
    }

    fn handle_start_tag(&mut self, tag: Tag, position: Option<Position>) {
        let node = match tag {
            Tag::Paragraph => Node {
                node_type: NodeType::Paragraph,
                position,
                ..Default::default()
            },
            Tag::Heading { level, .. } => Node {
                node_type: NodeType::Heading,
                depth: Some(self.heading_level_to_depth(level)),
                position,
                ..Default::default()
            },
            Tag::BlockQuote(_) => Node {
                node_type: NodeType::Blockquote,
                position,
                ..Default::default()
            },
            Tag::CodeBlock(kind) => {
                let (lang, meta) = match kind {
                    CodeBlockKind::Fenced(info) => {
                        let info_str = info.to_string();
                        let parts: Vec<&str> = info_str.splitn(2, ' ').collect();
                        let lang = if !parts[0].is_empty() {
                            Some(parts[0].to_string())
                        } else {
                            None
                        };
                        let meta = if parts.len() > 1 {
                            Some(parts[1].to_string())
                        } else {
                            None
                        };
                        (lang, meta)
                    }
                    CodeBlockKind::Indented => (None, None),
                };
                Node {
                    node_type: NodeType::Code,
                    lang,
                    meta,
                    position,
                    ..Default::default()
                }
            }
            Tag::List(start) => {
                let ordered = start.is_some();
                self.list_stack.push(ListContext {
                    ordered,
                    start,
                    tight: false, // TODO: Detect tight lists
                });
                Node {
                    node_type: NodeType::List,
                    ordered: Some(ordered),
                    start,
                    position,
                    ..Default::default()
                }
            }
            Tag::Item => Node {
                node_type: NodeType::ListItem,
                ordered: self.list_stack.last().map(|ctx| ctx.ordered),
                position,
                ..Default::default()
            },
            Tag::Emphasis => Node {
                node_type: NodeType::Emphasis,
                position,
                ..Default::default()
            },
            Tag::Strong => Node {
                node_type: NodeType::Strong,
                position,
                ..Default::default()
            },
            Tag::Strikethrough => Node {
                node_type: NodeType::Delete,
                position,
                ..Default::default()
            },
            Tag::Link {
                link_type,
                dest_url,
                title,
                ..
            } => Node {
                node_type: match link_type {
                    LinkType::Inline
                    | LinkType::Reference
                    | LinkType::Collapsed
                    | LinkType::Shortcut => NodeType::Link,
                    LinkType::Email => NodeType::Link,
                    LinkType::Autolink => NodeType::Link,
                    LinkType::ReferenceUnknown
                    | LinkType::CollapsedUnknown
                    | LinkType::ShortcutUnknown => NodeType::LinkReference,
                },
                url: Some(dest_url.to_string()),
                title: if title.is_empty() {
                    None
                } else {
                    Some(title.to_string())
                },
                position,
                ..Default::default()
            },
            Tag::Image {
                dest_url, title, ..
            } => Node {
                node_type: NodeType::Image,
                url: Some(dest_url.to_string()),
                title: if title.is_empty() {
                    None
                } else {
                    Some(title.to_string())
                },
                position,
                ..Default::default()
            },
            Tag::Table(alignment) => Node {
                node_type: NodeType::Table,
                align: Some(
                    alignment
                        .into_iter()
                        .map(|a| match a {
                            pulldown_cmark::Alignment::None => "none".to_string(),
                            pulldown_cmark::Alignment::Left => "left".to_string(),
                            pulldown_cmark::Alignment::Center => "center".to_string(),
                            pulldown_cmark::Alignment::Right => "right".to_string(),
                        })
                        .collect(),
                ),
                position,
                ..Default::default()
            },
            Tag::TableHead => Node {
                node_type: NodeType::TableRow,
                position,
                ..Default::default()
            },
            Tag::TableRow => Node {
                node_type: NodeType::TableRow,
                position,
                ..Default::default()
            },
            Tag::TableCell => Node {
                node_type: NodeType::TableCell,
                position,
                ..Default::default()
            },
            Tag::HtmlBlock => Node {
                node_type: NodeType::Html,
                position,
                ..Default::default()
            },
            Tag::MetadataBlock(_) => Node {
                node_type: NodeType::Yaml,
                position,
                ..Default::default()
            },
            _ => return, // Skip other tags
        };

        self.stack.push(node);
    }

    fn handle_end_tag(&mut self, tag: TagEnd) {
        if let Some(mut node) = self.stack.pop() {
            // Handle special cases
            match tag {
                TagEnd::CodeBlock => {
                    // Combine collected content for code blocks
                    if !self.content.is_empty() {
                        node.value = Some(self.content.join(""));
                        self.content.clear();
                    }
                }
                TagEnd::List(_) => {
                    self.list_stack.pop();
                }
                TagEnd::MetadataBlock(_) => {
                    // Store frontmatter content
                    if !self.content.is_empty() {
                        node.value = Some(self.content.join(""));
                        self.content.clear();
                    }
                }
                TagEnd::HtmlBlock => {
                    // Store HTML content
                    if !self.content.is_empty() {
                        node.value = Some(self.content.join(""));
                        self.content.clear();
                    }
                }
                _ => {}
            }

            // Add to parent or root
            if let Some(parent) = self.stack.last_mut() {
                parent.children.push(node);
            } else {
                self.root.children.push(node);
            }
        }
    }

    fn handle_text(&mut self, text: CowStr, position: Option<Position>) {
        // If we're in a code block or HTML block, collect the text
        if let Some(parent) = self.stack.last() {
            if matches!(
                parent.node_type,
                NodeType::Code | NodeType::Html | NodeType::Yaml
            ) {
                self.content.push(text.to_string());
                return;
            }
        }

        // Otherwise create a text node
        let node = Node {
            node_type: NodeType::Text,
            value: Some(text.to_string()),
            position,
            ..Default::default()
        };

        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(node);
        } else {
            self.root.children.push(node);
        }
    }

    fn handle_inline_code(&mut self, code: CowStr, position: Option<Position>) {
        let node = Node {
            node_type: NodeType::InlineCode,
            value: Some(code.to_string()),
            position,
            ..Default::default()
        };

        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(node);
        } else {
            self.root.children.push(node);
        }
    }

    fn handle_html(&mut self, html: CowStr, position: Option<Position>) {
        // If we're in an HTML block, collect the content
        if let Some(parent) = self.stack.last() {
            if parent.node_type == NodeType::Html {
                self.content.push(html.to_string());
                return;
            }
        }

        // Otherwise create an inline HTML node
        let node = Node {
            node_type: NodeType::Html,
            value: Some(html.to_string()),
            position,
            ..Default::default()
        };

        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(node);
        } else {
            self.root.children.push(node);
        }
    }

    fn handle_soft_break(&mut self, position: Option<Position>) {
        let node = Node {
            node_type: NodeType::Break,
            position,
            ..Default::default()
        };

        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(node);
        }
    }

    fn handle_hard_break(&mut self, position: Option<Position>) {
        let node = Node {
            node_type: NodeType::Break,
            position,
            ..Default::default()
        };

        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(node);
        }
    }

    fn handle_rule(&mut self, position: Option<Position>) {
        let node = Node {
            node_type: NodeType::ThematicBreak,
            position,
            ..Default::default()
        };

        self.root.children.push(node);
    }

    fn handle_task_list_marker(&mut self, checked: bool) {
        if let Some(parent) = self.stack.last_mut() {
            if parent.node_type == NodeType::ListItem {
                parent.checked = Some(checked);
            }
        }
    }

    fn handle_inline_math(&mut self, math: CowStr, position: Option<Position>) {
        let node = Node {
            node_type: NodeType::InlineMath,
            value: Some(math.to_string()),
            position,
            ..Default::default()
        };

        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(node);
        } else {
            self.root.children.push(node);
        }
    }

    fn handle_display_math(&mut self, math: CowStr, position: Option<Position>) {
        let node = Node {
            node_type: NodeType::Math,
            value: Some(math.to_string()),
            position,
            ..Default::default()
        };

        self.root.children.push(node);
    }

    fn heading_level_to_depth(&self, level: HeadingLevel) -> u8 {
        match level {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
        }
    }

    fn range_to_position(&self, range: std::ops::Range<usize>, source: &str) -> Position {
        let start_offset = range.start;
        let end_offset = range.end;

        // Calculate line and column for start
        let mut line = 1;
        let mut column = 1;
        let mut current_offset = 0;

        for ch in source.chars() {
            if current_offset >= start_offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            current_offset += ch.len_utf8();
        }

        let start_line = line;
        let start_column = column;

        // Continue for end position
        for ch in source[current_offset..].chars() {
            if current_offset >= end_offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            current_offset += ch.len_utf8();
        }

        Position {
            start: crate::position::Point {
                line: start_line,
                column: start_column,
                offset: start_offset,
            },
            end: crate::position::Point {
                line,
                column,
                offset: end_offset,
            },
            source: None,
        }
    }

    fn finish(self) -> Node {
        self.root
    }
}
