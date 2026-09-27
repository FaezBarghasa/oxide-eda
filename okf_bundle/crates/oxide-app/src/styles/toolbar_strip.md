---
okf_version: "0.2"
type: Function
title: toolbar_strip
description: Toolbar / menu bar strip
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/toolbar_strip
language: rust
---

# toolbar_strip

Toolbar / menu bar strip

## Signature

```rust
pub fn toolbar_strip(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Toolbar / menu bar strip

## Source
Lines 50–64 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_main_window_chrome](/crates/oxide-app/src/app/view/chrome/view_main_window_chrome.md) |
| called_by | [view_bom_preview_body_inner](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview_body_inner.md) |
| called_by | [bom_table](/crates/oxide-app/src/app/view/dialogs/bom/table/bom_table.md) |
| called_by | [view_main_for](/crates/oxide-app/src/app/view/mod/view_main_for.md) |
| called_by | [view](/crates/oxide-app/src/find_replace/view.md) |
| called_by | [wrap_plain](/crates/oxide-app/src/menu_bar/mod/wrap_plain.md) |
| called_by | [view](/crates/oxide-app/src/tab_bar/view.md) |
