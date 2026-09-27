---
okf_version: "0.2"
type: Function
title: iced_font_for_family
description: "Build an [`iced::Font`] that targets the given family name. iced's"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/iced_font_for_family
language: rust
---

# iced_font_for_family

Build an [`iced::Font`] that targets the given family name. iced's

## Signature

```rust
pub fn iced_font_for_family(name: &str) -> iced::Font
```

## Visibility

- `pub`

## Docstring

Build an [`iced::Font`] that targets the given family name. iced's
`Font::with_name` requires `&'static str` because the renderer
caches by family name, but the Preferences panel hands us font
names as runtime `String`s. The intern map below leaks one
`&'static str` per unique family name ever resolved during a
session — bounded by the small set of fonts a user actually picks,
so the cumulative leak is negligible. Used for surfaces that need
the canvas font (Iosevka by default) — e.g. the symbol hover
tooltip.

## Source
Lines 132–152 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| called_by | [view_hover_tooltip](/crates/oxide-app/src/app/view/overlays/mod/view_hover_tooltip.md) |
