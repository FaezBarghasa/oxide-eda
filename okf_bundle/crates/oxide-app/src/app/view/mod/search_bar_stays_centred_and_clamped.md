---
okf_version: "0.2"
type: Function
title: search_bar_stays_centred_and_clamped
description: "The bar is centred on the window, shrinks monotonically as the"
resource: crates/oxide-app/src/app/view/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/mod/search_bar_stays_centred_and_clamped
language: rust
---

# search_bar_stays_centred_and_clamped

The bar is centred on the window, shrinks monotonically as the

## Signature

```rust
fn search_bar_stays_centred_and_clamped()
```

## Decorators

- `test`

## Docstring

The bar is centred on the window, shrinks monotonically as the
window narrows, and never runs off the right edge or under the
menu items.
[test]

## Source
Lines 759–784 in `crates/oxide-app/src/app/view/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/app/view/mod.md) |
| calls | [approx_menu_bar_width](/crates/oxide-app/src/menu_bar/mod/approx_menu_bar_width.md) |
| calls | [chrome_search_bar_geometry](/crates/oxide-app/src/app/view/mod/chrome_search_bar_geometry.md) |
