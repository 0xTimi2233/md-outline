use crate::parser::HeadingItem;

/// Formats a list of HeadingItems into a human-readable ASCII/Unicode tree with aligned line numbers.
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
            "L{:<4} {}{}",
            current.line, tree_prefix, current.text
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_tree_empty() {
        let result = format_tree(&[]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_format_tree_hierarchy() {
        let headings = vec![
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
        ];

        let result = format_tree(&headings);
        let expected = "\
L1    Root
L5    ├── Child A
L8    │   ├── Grandchild A1
L11   │   └── Grandchild A2
L18   └── Child B";

        assert_eq!(result, expected);
    }
}
