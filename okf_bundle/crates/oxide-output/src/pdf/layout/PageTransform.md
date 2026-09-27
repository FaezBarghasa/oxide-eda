---
okf_version: "0.2"
type: Class
title: PageTransform
description: Resolved coordinate transform for a single page.
resource: crates/oxide-output/src/pdf/layout.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/layout/PageTransform
language: rust
---

# PageTransform

Resolved coordinate transform for a single page.

## Signature

```rust
pub struct PageTransform
```

## Decorators

- `derive(Debug, Clone, Copy)`

## Visibility

- `pub`

## Docstring

Resolved coordinate transform for a single page.

Converts schematic coordinates (mm, top-left origin, Y down) to the
layout coordinate space (any unit, origin and axis direction
determined by the caller).
[derive(Debug, Clone, Copy)]

## Methods

- `mm_to_unit`
- `translate_x`
- `translate_y`

## Source
Lines 75–82 in `crates/oxide-output/src/pdf/layout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layout](/crates/oxide-output/src/pdf/layout.md) |
