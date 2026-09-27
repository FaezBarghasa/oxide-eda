---
okf_version: "0.2"
type: Function
title: view_command_palette_dropdown
description: Result list for the chrome-strip command palette. Anchored
resource: crates/oxide-app/src/app/view/overlays/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/mod/view_command_palette_dropdown
language: rust
---

# view_command_palette_dropdown

Result list for the chrome-strip command palette. Anchored

## Signature

```rust
impl Oxide { pub(super) fn view_command_palette_dropdown(&self) -> Element<'_, Message> }
```

## Visibility

- `pub(super)`

## Docstring

Result list for the chrome-strip command palette. Anchored
below the chrome strip; scoring + ranking happens in the
`command_palette` module so this view stays a thin renderer.

## Source
Lines 142–275 in `crates/oxide-app/src/app/view/overlays/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/app/view/overlays/mod.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [build_catalog](/crates/oxide-app/src/app/command_palette/build_catalog.md) |
| calls | [rank_results](/crates/oxide-app/src/app/command_palette/rank_results.md) |
| calls | [chrome_search_bar_geometry](/crates/oxide-app/src/app/view/mod/chrome_search_bar_geometry.md) |
