---
okf_version: "0.2"
type: Function
title: tree_row
resource: crates/chrome-catalog/src/project_tree.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:chrome-catalog"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/chrome-catalog/src/project_tree/tree_row
language: rust
---

# tree_row

## Signature

```rust
fn tree_row(
    label: &str,
    is_open: bool,
    is_dirty: bool,
    is_active: bool,
    tokens: &ThemeTokens,
) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Source
Lines 22–68 in `crates/chrome-catalog/src/project_tree.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_tree](/crates/chrome-catalog/src/project_tree.md) |
| calls | [color](/crates/chrome-catalog/src/theme/color.md) |
| called_by | [view](/crates/chrome-catalog/src/project_tree/view.md) |
