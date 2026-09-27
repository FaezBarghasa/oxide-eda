---
okf_version: "0.2"
type: Function
title: view_section
description: "Render a single mount-source section with its collapsible header,"
resource: crates/oxide-app/src/panels/components_panel/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/mod/view_section
language: rust
---

# view_section

Render a single mount-source section with its collapsible header,

## Signature

```rust
fn view_section(
    src: ComponentsMountSource,
    state: &'a LibraryState,
    libs_for_source: &[&'a OpenLibrary],
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

Render a single mount-source section with its collapsible header,
the `+ Add Library…` action (Installed/Global only), and a
per-library list of components matching the filter.

## Source
Lines 136–227 in `crates/oxide-app/src/panels/components_panel/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components_panel](/crates/oxide-app/src/panels/components_panel/mod.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [chevron](/crates/oxide-app/src/panels/components_panel/mod/chevron.md) |
| calls | [menu_item](/crates/oxide-app/src/styles/menu_item.md) |
| calls | [view_library_block](/crates/oxide-app/src/panels/components_panel/mod/view_library_block.md) |
| calls | [thin_sep](/crates/oxide-app/src/panels/components_panel/mod/thin_sep.md) |
| called_by | [view](/crates/oxide-app/src/panels/components_panel/mod/view.md) |
