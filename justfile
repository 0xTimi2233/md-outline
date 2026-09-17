default:
    @just --list

ci: check lint test audit

# Code formatting check
check:
    cargo fmt --check

# Code formatting apply
fmt:
    cargo fmt --all

# Clippy linter static analysis
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
test:
    cargo test --all-features

# Security and dependency audit
audit:
    cargo deny check advisories bans sources

# Build optimized release binary locally
build:
    cargo build --release

# Synchronize project version (Example: just bump 0.1.1)
bump version:
    @bash scripts/bump.sh {{version}}

# Download and install latest release binary from GitHub to ~/.local/bin
update:
    @bash scripts/update.sh

# Sync GitHub repository standard labels
sync-labels repo:
    #!/usr/bin/env bash
    set -euo pipefail
    manifest="{{justfile_directory()}}/.github/labels.json"
    wanted=$(mktemp)
    jq -r '.[].name' "$manifest" | sort > "$wanted"
    gh label list --repo "{{repo}}" --json name --jq '.[].name' | sort | comm -13 "$wanted" - | while read -r name; do
        gh label delete "$name" --repo "{{repo}}" --yes
    done
    rm -f "$wanted"
    jq -c '.[]' "$manifest" | while read -r item; do
        name=$(echo "$item" | jq -r .name)
        color=$(echo "$item" | jq -r .color)
        desc=$(echo "$item" | jq -r .description)
        gh label create "$name" --color "$color" --description "$desc" --repo "{{repo}}" --force
    done

# Sync GitHub repository branch protection ruleset
sync-ruleset repo:
    #!/usr/bin/env bash
    set -euo pipefail
    ruleset_file="{{justfile_directory()}}/.github/ruleset.json"
    existing_id=$(gh api "repos/{{repo}}/rulesets" --jq '.[] | select(.name=="default-branch-protection") | .id' || true)
    if [ -n "$existing_id" ]; then
        echo "更新已有 Ruleset (ID: $existing_id)..."
        gh api --method PUT "repos/{{repo}}/rulesets/$existing_id" --input "$ruleset_file" > /dev/null
    else
        echo "创建新 Ruleset..."
        gh api --method POST "repos/{{repo}}/rulesets" --input "$ruleset_file" > /dev/null
    fi
    echo "分支保护规则集已成功同步至 {{repo}}"
