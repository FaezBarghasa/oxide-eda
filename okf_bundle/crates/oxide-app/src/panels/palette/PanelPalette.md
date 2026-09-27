---
okf_version: "0.2"
type: Class
title: PanelPalette
description: Every theme colour the Properties panel and its sub-forms render with.
resource: crates/oxide-app/src/panels/palette.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/palette/PanelPalette
language: rust
---

# PanelPalette

Every theme colour the Properties panel and its sub-forms render with.

## Signature

```rust
pub struct PanelPalette
```

## Decorators

- `derive(Debug, Clone, Copy)`

## Visibility

- `pub`

## Docstring

Every theme colour the Properties panel and its sub-forms render with.

Built once per frame by `panels::properties::view_properties` and
`properties_parameters::general::view_properties_general`, then passed
down by value — all eight fields are `Copy`, so each hop costs a move
rather than a fresh borrow of the tokens.
[derive(Debug, Clone, Copy)]

## Methods

- `muted`
- `primary`
- `border`
- `input_bg`
- `input_bdr`
- `accent`
- `tag_hover`
- `seg_hover`

## Source
Lines 30–48 in `crates/oxide-app/src/panels/palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [palette](/crates/oxide-app/src/panels/palette.md) |
