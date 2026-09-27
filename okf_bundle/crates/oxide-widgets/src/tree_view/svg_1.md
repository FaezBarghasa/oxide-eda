---
okf_version: "0.2"
type: Function
title: svg
description: Return the cached SVG handle for this icon. Each variant
resource: crates/oxide-widgets/src/tree_view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/tree_view/svg_1
language: rust
---

# svg

Return the cached SVG handle for this icon. Each variant

## Signature

```rust
pub fn svg(self) -> svg::Handle
```

## Visibility

- `pub`

## Docstring

Return the cached SVG handle for this icon. Each variant
memoises its handle through a `OnceLock` so bytes are only
wrapped once per process — subsequent calls are a cheap
`Arc::clone`.

## Source
Lines 142–172 in `crates/oxide-widgets/src/tree_view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tree_view](/crates/oxide-widgets/src/tree_view.md) |
