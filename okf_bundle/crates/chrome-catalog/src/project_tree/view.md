---
okf_version: "0.2"
type: Function
title: view
resource: crates/chrome-catalog/src/project_tree.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:chrome-catalog"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/chrome-catalog/src/project_tree/view
language: rust
---

# view

## Signature

```rust
pub(crate) fn view(tokens: &ThemeTokens) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Visibility

- `pub(crate)`

## Source
Lines 8–20 in `crates/chrome-catalog/src/project_tree.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_tree](/crates/chrome-catalog/src/project_tree.md) |
| calls | [tree_row](/crates/chrome-catalog/src/project_tree/tree_row.md) |
