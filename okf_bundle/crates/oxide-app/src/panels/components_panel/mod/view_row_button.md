---
okf_version: "0.2"
type: Function
title: view_row_button
description: "One row button — fires `OpenComponentRow` on click. Mirrors the"
resource: crates/oxide-app/src/panels/components_panel/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/mod/view_row_button
language: rust
---

# view_row_button

One row button — fires `OpenComponentRow` on click. Mirrors the

## Signature

```rust
fn view_row_button(
    library_path: PathBuf,
    table: String,
    row_data: &'a ComponentRow,
    text_c: Color,
    muted: Color,
    hover_c: Color,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

One row button — fires `OpenComponentRow` on click. Mirrors the
row layout of the existing legacy Components panel so users see
the same `(internal_pn — mpn)  manufacturer` shape.

## Source
Lines 314–365 in `crates/oxide-app/src/panels/components_panel/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components_panel](/crates/oxide-app/src/panels/components_panel/mod.md) |
| called_by | [view_library_block](/crates/oxide-app/src/panels/components_panel/mod/view_library_block.md) |
