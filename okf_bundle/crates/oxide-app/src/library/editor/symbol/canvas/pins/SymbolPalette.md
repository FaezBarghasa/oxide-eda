---
okf_version: "0.2"
type: Class
title: SymbolPalette
description: Palette derived from the active sheet colour — picks a content
resource: crates/oxide-app/src/library/editor/symbol/canvas/pins.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/pins/SymbolPalette
language: rust
---

# SymbolPalette

Palette derived from the active sheet colour — picks a content

## Signature

```rust
pub(super) struct SymbolPalette
```

## Visibility

- `pub(super)`

## Docstring

Palette derived from the active sheet colour — picks a content
foreground that reads correctly on the sheet bg. Two flavours:
dark-on-light (Cream / White / LightGray) and light-on-dark
(Black / DarkGray). Mirrors Altium's per-sheet contrast rule.

## Methods

- `body`
- `pin`
- `text`
- `grid`
- `axis`

## Source
Lines 15–22 in `crates/oxide-app/src/library/editor/symbol/canvas/pins.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pins](/crates/oxide-app/src/library/editor/symbol/canvas/pins.md) |
