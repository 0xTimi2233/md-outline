use crate::args::OutputFormat;
use crate::parser::HeadingItem;

/// Formats headings according to the specified output format.
pub fn format_headings(
    headings: &[HeadingItem],
    format: OutputFormat,
) -> Result<String, serde_json::Error> {
    match format {
        OutputFormat::Markdown => Ok(format_markdown(headings)),
        OutputFormat::Tree => Ok(format_tree(headings)),
        OutputFormat::Json => format_json(headings),
    }
}

/// Formats headings into standard Markdown syntax with 6-digit right-aligned line numbers.
pub fn format_markdown(headings: &[HeadingItem]) -> String {
    if headings.is_empty() {
        return String::new();
    }

    let mut lines = Vec::with_capacity(headings.len());

    for current in headings {
        let hashes = "#".repeat(current.level as usize);
        lines.push(format!("{:>6}: {} {}", current.line, hashes, current.text));
    }

    lines.join("\n")
}

/// Formats headings into a human-readable Unicode tree with aligned line numbers.
pub fn format_tree(headings: &[HeadingItem]) -> String {
    if headings.is_empty() {
        return String::new();
    }

    let min_level = headings.iter().map(|h| h.level).min().unwrap_or(1);
    let mut lines = Vec::with_capacity(headings.len());

    for (i, current) in headings.iter().enumerate() {
        let depth = (current.level.saturating_sub(min_level)) as usize;

        let has_sibling_at_level = |target_depth: usize| -> bool {
            for next in &headings[(i + 1)..] {
                let next_depth = (next.level.saturating_sub(min_level)) as usize;
                if next_depth == target_depth {
                    return true;
                }
                if next_depth < target_depth {
                    return false;
                }
            }
            false
        };

        let mut tree_prefix = String::new();
        for l in 0..depth {
            if l == depth - 1 {
                if has_sibling_at_level(depth) {
                    tree_prefix.push_str("├── ");
                } else {
                    tree_prefix.push_str("└── ");
                }
            } else if has_sibling_at_level(l + 1) {
                tree_prefix.push_str("│   ");
            } else {
                tree_prefix.push_str("    ");
            }
        }

        lines.push(format!(
            "{:>6}:  {}{}",
            current.line, tree_prefix, current.text
        ));
    }

    lines.join("\n")
}

/// Formats headings into a formatted JSON array string.
pub fn format_json(headings: &[HeadingItem]) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(headings)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_headings() -> Vec<HeadingItem> {
        vec![
            HeadingItem {
                level: 1,
                line: 1,
                text: "Root".to_string(),
            },
            HeadingItem {
                level: 2,
                line: 5,
                text: "Child A".to_string(),
            },
            HeadingItem {
                level: 3,
                line: 8,
                text: "Grandchild A1".to_string(),
            },
            HeadingItem {
                level: 3,
                line: 11,
                text: "Grandchild A2".to_string(),
            },
            HeadingItem {
                level: 2,
                line: 18,
                text: "Child B".to_string(),
            },
        ]
    }

    #[test]
    fn test_format_markdown_empty() {
        assert_eq!(format_markdown(&[]), "");
    }

    #[test]
    fn test_format_markdown_hierarchy() {
        let headings = sample_headings();
        let result = format_markdown(&headings);
        let expected = [
            "     1: # Root",
            "     5: ## Child A",
            "     8: ### Grandchild A1",
            "    11: ### Grandchild A2",
            "    18: ## Child B",
        ]
        .join("\n");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_tree_empty() {
        assert_eq!(format_tree(&[]), "");
    }

    #[test]
    fn test_format_tree_hierarchy() {
        let headings = sample_headings();
        let result = format_tree(&headings);
        let expected = [
            "     1:  Root",
            "     5:  ├── Child A",
            "     8:  │   ├── Grandchild A1",
            "    11:  │   └── Grandchild A2",
            "    18:  └── Child B",
        ]
        .join("\n");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_json_empty() {
        let result = format_json(&[]).unwrap_or_default();
        assert_eq!(result, "[]");
    }

    #[test]
    fn test_format_json_hierarchy() {
        let headings = sample_headings();
        let json_str = format_json(&headings).unwrap_or_default();
        let deserialized: Vec<HeadingItem> = serde_json::from_str(&json_str).unwrap_or_default();
        assert_eq!(deserialized, headings);
    }

    #[test]
    fn test_format_headings_router() {
        let headings = sample_headings();
        assert_eq!(
            format_headings(&headings, OutputFormat::Markdown).unwrap_or_default(),
            format_markdown(&headings)
        );
        assert_eq!(
            format_headings(&headings, OutputFormat::Tree).unwrap_or_default(),
            format_tree(&headings)
        );
        assert_eq!(
            format_headings(&headings, OutputFormat::Json).unwrap_or_default(),
            format_json(&headings).unwrap_or_default()
        );
    }
}
