---
okf_version: "0.2"
type: Function
title: collect_rust_files
resource: crates/oxide-renderer/tests/no_literal_colors.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/tests/no_literal_colors/collect_rust_files
language: rust
---

# collect_rust_files

## Signature

```rust
fn collect_rust_files(root: &Path) -> Vec<PathBuf>
```

## Source
Lines 11–35 in `crates/oxide-renderer/tests/no_literal_colors.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [no_literal_colors](/crates/oxide-renderer/tests/no_literal_colors.md) |
| calls | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
| called_by | [renderer_runtime_source_rejects_literal_color_blocks](/crates/oxide-renderer/tests/no_literal_colors/renderer_runtime_source_rejects_literal_color_blocks.md) |
