use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

const SAMPLE_MARKDOWN: &str = r#"# Root Title

Intro paragraph.

## Section Alpha
Description alpha.

### Subsection Alpha 1
Detail 1.

### Subsection Alpha 2
Detail 2.

```python
# This is a comment inside code block, not heading
```

## Section Beta
Description beta.

Setext Sub Heading
------------------
Content.

#### Level 4 Deep Detail
Deep content.
"#;

#[test]
fn test_cli_default_indent_format_success() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;
    file.write_all(SAMPLE_MARKDOWN.as_bytes())?;

    let mut cmd = Command::cargo_bin("md-outline")?;
    cmd.arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("1: Root Title"))
        .stdout(predicate::str::contains("5:   Section Alpha"))
        .stdout(predicate::str::contains("8:     Subsection Alpha 1"))
        .stdout(predicate::str::contains("11:     Subsection Alpha 2"))
        .stdout(predicate::str::contains("18:   Section Beta"))
        .stdout(predicate::str::contains("21:   Setext Sub Heading"))
        .stdout(predicate::str::contains("25:       Level 4 Deep Detail"));

    Ok(())
}

#[test]
fn test_cli_tree_format_success() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;
    file.write_all(SAMPLE_MARKDOWN.as_bytes())?;

    let mut cmd = Command::cargo_bin("md-outline")?;
    cmd.arg(file.path())
        .arg("-f")
        .arg("tree")
        .assert()
        .success()
        .stdout(predicate::str::contains("1:   Root Title"))
        .stdout(predicate::str::contains("5:   ├── Section Alpha"))
        .stdout(predicate::str::contains("8:   │   ├── Subsection Alpha 1"))
        .stdout(predicate::str::contains("11:  │   └── Subsection Alpha 2"))
        .stdout(predicate::str::contains("18:  ├── Section Beta"))
        .stdout(predicate::str::contains("21:  └── Setext Sub Heading"))
        .stdout(predicate::str::contains(
            "25:          └── Level 4 Deep Detail",
        ));

    Ok(())
}

#[test]
fn test_cli_json_format_success() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;
    file.write_all(SAMPLE_MARKDOWN.as_bytes())?;

    let mut cmd = Command::cargo_bin("md-outline")?;
    cmd.arg(file.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""level": 1"#))
        .stdout(predicate::str::contains(r#""line": 1"#))
        .stdout(predicate::str::contains(r#""text": "Root Title""#))
        .stdout(predicate::str::contains(r#""text": "Section Alpha""#));

    Ok(())
}

#[test]
fn test_cli_depth_filter_success() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;
    file.write_all(SAMPLE_MARKDOWN.as_bytes())?;

    let mut cmd = Command::cargo_bin("md-outline")?;
    cmd.arg(file.path())
        .arg("-d")
        .arg("2")
        .assert()
        .success()
        .stdout(predicate::str::contains("1: Root Title"))
        .stdout(predicate::str::contains("5:   Section Alpha"))
        .stdout(predicate::str::contains("18:   Section Beta"))
        .stdout(predicate::str::contains("21:   Setext Sub Heading"))
        .stdout(predicate::str::contains("Subsection Alpha 1").not())
        .stdout(predicate::str::contains("Level 4 Deep Detail").not());

    Ok(())
}

#[test]
fn test_cli_stdin_pipe_without_dash_success() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("md-outline")?;
    cmd.write_stdin(SAMPLE_MARKDOWN)
        .assert()
        .success()
        .stdout(predicate::str::contains("1: Root Title"))
        .stdout(predicate::str::contains("5:   Section Alpha"));

    Ok(())
}

#[test]
fn test_cli_stdin_pipe_with_dash_success() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("md-outline")?;
    cmd.arg("-")
        .write_stdin(SAMPLE_MARKDOWN)
        .assert()
        .success()
        .stdout(predicate::str::contains("1: Root Title"))
        .stdout(predicate::str::contains("5:   Section Alpha"));

    Ok(())
}

#[test]
fn test_cli_file_not_found_fails() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("md-outline")?;
    cmd.arg("non_existent_file_12345.md").assert().failure();

    Ok(())
}

#[test]
fn test_cli_invalid_format_fails() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;
    file.write_all(SAMPLE_MARKDOWN.as_bytes())?;

    let mut cmd = Command::cargo_bin("md-outline")?;
    cmd.arg(file.path())
        .arg("-f")
        .arg("unknown-format")
        .assert()
        .failure();

    Ok(())
}
