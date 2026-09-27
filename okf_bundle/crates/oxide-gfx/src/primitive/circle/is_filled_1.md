---
okf_version: "0.2"
type: Function
title: is_filled
description: Whether this circle renders as a filled disc (rather than a stroked
resource: crates/oxide-gfx/src/primitive/circle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/primitive/circle/is_filled_1
language: rust
---

# is_filled

Whether this circle renders as a filled disc (rather than a stroked

## Signature

```rust
pub fn is_filled(&self) -> bool
```

## Visibility

- `pub`

## Docstring

Whether this circle renders as a filled disc (rather than a stroked
ring). A non-positive `stroke_width` means "fill"; any positive width
strokes a ring of that width. Shared CPU↔GPU predicate the parity test
locks against.

## Source
Lines 22–24 in `crates/oxide-gfx/src/primitive/circle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [circle](/crates/oxide-gfx/src/primitive/circle.md) |
