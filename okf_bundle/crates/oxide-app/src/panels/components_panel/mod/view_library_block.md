---
okf_version: "0.2"
type: Function
title: view_library_block
description: "Render one mounted library inside its source section: a small"
resource: crates/oxide-app/src/panels/components_panel/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/mod/view_library_block
language: rust
---

# view_library_block

Render one mounted library inside its source section: a small

## Signature

```rust
fn view_library_block(
    lib: &'a OpenLibrary,
    needle: &str,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

Render one mounted library inside its source section: a small
header row with the library name + a row count, then a flat list
of every component matching the panel-wide filter. Clicking a row
fires `LibraryMessage::OpenComponentRow` so the existing Component
Preview tab opens — same flow the project tree uses.

## Source
Lines 234–294 in `crates/oxide-app/src/panels/components_panel/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components_panel](/crates/oxide-app/src/panels/components_panel/mod.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [row_matches](/crates/oxide-app/src/panels/components_panel/mod/row_matches.md) |
| calls | [view_row_button](/crates/oxide-app/src/panels/components_panel/mod/view_row_button.md) |
| called_by | [view_section](/crates/oxide-app/src/panels/components_panel/mod/view_section.md) |
