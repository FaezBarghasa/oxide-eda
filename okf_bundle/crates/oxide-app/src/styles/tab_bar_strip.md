---
okf_version: "0.2"
type: Function
title: tab_bar_strip
description: Tab bar background (dock region header)
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/tab_bar_strip
language: rust
---

# tab_bar_strip

Tab bar background (dock region header)

## Signature

```rust
pub fn tab_bar_strip(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Tab bar background (dock region header)

## Source
Lines 141–155 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_header](/crates/oxide-app/src/library/browser/header/view_header.md) |
| called_by | [view_tabs](/crates/oxide-app/src/library/editor/mod/view_tabs.md) |
| called_by | [view_footprint_layers_strip](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_layers_strip.md) |
| called_by | [view_footprint_sketch_toolbar](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_sketch_toolbar.md) |
| called_by | [view_footprint_toolbar](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_toolbar.md) |
| called_by | [view_footprint_top_strip](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_top_strip.md) |
| called_by | [view_symbol_toolbar](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_toolbar.md) |
