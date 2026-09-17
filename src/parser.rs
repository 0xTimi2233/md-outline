use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Represents a parsed Markdown heading with depth, 1-based line number, and clean text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadingItem {
    pub level: u8,
    pub line: usize,
    pub text: String,
}

/// Parses Markdown content and extracts headings within the specified maximum depth.
pub fn parse_headings(content: &str, max_depth: u8) -> Vec<HeadingItem> {
    let mut line_starts = vec![0];
    for (i, b) in content.bytes().enumerate() {
        if b == b'\n' {
            line_starts.push(i + 1);
        }
    }

    let get_line = |offset: usize| -> usize {
        match line_starts.binary_search(&offset) {
            Ok(idx) => idx + 1,
            Err(idx) => idx,
        }
    };

    let parser = Parser::new_ext(content, Options::all()).into_offset_iter();
    let mut headings = Vec::new();
    let mut current_heading: Option<(HeadingLevel, usize, String)> = None;

    for (event, range) in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let line = get_line(range.start);
                current_heading = Some((level, line, String::new()));
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some((_, _, ref mut title)) = current_heading {
                    title.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, line, title)) = current_heading.take() {
                    let level_u8 = heading_level_to_u8(level);
                    if level_u8 <= max_depth {
                        headings.push(HeadingItem {
                            level: level_u8,
                            line,
                            text: title.trim().to_string(),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    headings
}

fn heading_level_to_u8(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_headings_various_styles_and_code_blocks() {
        let markdown = r#"# Main Title

Some text.

## Level 2 with [Link Text](https://example.com) and `inline code`

```bash
# This is a comment inside code block
```

~~~python
# Tilde code block comment
~~~

    # Indented code block comment

Setext Level One
================

Setext Level Two
----------------

### Level 3 Title
"#;

        let result = parse_headings(markdown, 6);
        let expected = vec![
            HeadingItem {
                level: 1,
                line: 1,
                text: "Main Title".to_string(),
            },
            HeadingItem {
                level: 2,
                line: 5,
                text: "Level 2 with Link Text and inline code".to_string(),
            },
            HeadingItem {
                level: 1,
                line: 17,
                text: "Setext Level One".to_string(),
            },
            HeadingItem {
                level: 2,
                line: 20,
                text: "Setext Level Two".to_string(),
            },
            HeadingItem {
                level: 3,
                line: 23,
                text: "Level 3 Title".to_string(),
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_headings_depth_filter() {
        let markdown = "# H1\n## H2\n### H3\n";
        let result = parse_headings(markdown, 2);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].text, "H1");
        assert_eq!(result[1].text, "H2");
    }
}
