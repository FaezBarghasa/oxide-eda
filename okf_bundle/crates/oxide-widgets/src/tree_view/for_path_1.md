---
okf_version: "0.2"
type: Function
title: for_path
description: "Pick a `TreeIcon` for a filename. Both Oxide `.snx***` and"
resource: crates/oxide-widgets/src/tree_view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/tree_view/for_path_1
language: rust
---

# for_path

Pick a `TreeIcon` for a filename. Both Oxide `.snx***` and

## Signature

```rust
pub fn for_path(filename: &str) -> Self
```

## Visibility

- `pub`

## Docstring

Pick a `TreeIcon` for a filename. Both Oxide `.snx***` and
Standard `.standard_*` extensions route to the same Oxide-brand
glyph family so the project tree reads as one cohesive visual
family regardless of whether the underlying file is native or
Standard. Unknown extensions fall back to `File`.

## Source
Lines 179–209 in `crates/oxide-widgets/src/tree_view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tree_view](/crates/oxide-widgets/src/tree_view.md) |
