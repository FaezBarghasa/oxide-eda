---
okf_version: "0.2"
type: Function
title: view
description: "Render the Components Panel. Returns a `LibraryMessage`-typed"
resource: crates/oxide-app/src/panels/components_panel/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/mod/view
language: rust
---

# view

Render the Components Panel. Returns a `LibraryMessage`-typed

## Signature

```rust
pub fn view(
    state: &'a LibraryState,
    ctx: &'a crate::panels::PanelContext,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the Components Panel. Returns a `LibraryMessage`-typed
element; the dock host wraps it in `DockMessage::Library` to route
through the existing library dispatcher.

The Project mount-source bucket reads through
`ctx.projects[].libraries[].root` rather than threading a
separate slice. The panel context already carries the project
info, so the dock's `view_region` signature stays narrow.

## Source
Lines 67–131 in `crates/oxide-app/src/panels/components_panel/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components_panel](/crates/oxide-app/src/panels/components_panel/mod.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [view_section](/crates/oxide-app/src/panels/components_panel/mod/view_section.md) |
| calls | [thin_sep](/crates/oxide-app/src/panels/components_panel/mod/thin_sep.md) |
