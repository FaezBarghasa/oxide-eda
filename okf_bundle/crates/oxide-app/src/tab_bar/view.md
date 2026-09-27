---
okf_version: "0.2"
type: Function
title: view
resource: crates/oxide-app/src/tab_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/tab_bar/view
language: rust
---

# view

## Signature

```rust
pub fn view(
    tabs: &[TabInfo],
    active: usize,
    dragging: Option<usize>,
    visible_paths: &std::collections::HashSet<std::path::PathBuf>,
    tokens: &ThemeTokens,
) -> Element<'a, TabMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 29–140 in `crates/oxide-app/src/tab_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tab_bar](/crates/oxide-app/src/tab_bar.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [pill_fill](/crates/oxide-app/src/tab_bar/pill_fill.md) |
| calls | [toolbar_strip](/crates/oxide-app/src/styles/toolbar_strip.md) |
