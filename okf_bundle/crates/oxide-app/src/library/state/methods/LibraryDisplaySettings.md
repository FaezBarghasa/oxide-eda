---
okf_version: "0.2"
type: Class
title: LibraryDisplaySettings
description: Per-library canvas + UI defaults shared across every primitive
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/LibraryDisplaySettings
language: rust
---

# LibraryDisplaySettings

Per-library canvas + UI defaults shared across every primitive

## Signature

```rust
pub struct LibraryDisplaySettings
```

## Decorators

- `derive(Debug, Clone, Copy)`

## Visibility

- `pub`

## Docstring

Per-library canvas + UI defaults shared across every primitive
editor tab opened from the same `.snxlib`. See [`OpenLibrary::display`].
[derive(Debug, Clone, Copy)]

## Methods

- `unit`
- `grid_size_mm`
- `grid_visible`
- `sheet_color`
- `pin_selection`

## Source
Lines 405–422 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
