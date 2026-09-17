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
fn test_cli_full_outline_success() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;
    file.write_all(SAMPLE_MARKDOWN.as_bytes())?;

    let mut cmd = Command::cargo_bin("md-outline")?;
    cmd.arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("L1    Root Title"))
        .stdout(predicate::str::contains("L5    ├── Section Alpha"))
        .stdout(predicate::str::contains("L8    │   ├── Subsection Alpha 1"))
        .stdout(predicate::str::contains("L11   │   └── Subsection Alpha 2"))
        .stdout(predicate::str::contains("L18   ├── Section Beta"))
        .stdout(predicate::str::contains("L21   └── Setext Sub Heading"))
        .stdout(predicate::str::contains(
            "L25           └── Level 4 Deep Detail",
        ));

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
        .stdout(predicate::str::contains("L1    Root Title"))
        .stdout(predicate::str::contains("L5    ├── Section Alpha"))
        .stdout(predicate::str::contains("L18   ├── Section Beta"))
        .stdout(predicate::str::contains("L21   └── Setext Sub Heading"))
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
        .stdout(predicate::str::contains("L1    Root Title"))
        .stdout(predicate::str::contains("L5    ├── Section Alpha"));

    Ok(())
}

#[test]
fn test_cli_stdin_pipe_with_dash_success() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("md-outline")?;
    cmd.arg("-")
        .write_stdin(SAMPLE_MARKDOWN)
        .assert()
        .success()
        .stdout(predicate::str::contains("L1    Root Title"))
        .stdout(predicate::str::contains("L5    ├── Section Alpha"));

    Ok(())
}

#[test]
fn test_cli_file_not_found_fails() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("md-outline")?;
    cmd.arg("non_existent_file_12345.md").assert().failure();

    Ok(())
}
