---
okf_version: "0.2"
type: Function
title: dimensions_mm
description: "Effective `(width_mm, height_mm)` honouring orientation. Custom"
resource: crates/oxide-output/src/pdf/page.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/pdf/page/dimensions_mm_1
language: rust
---

# dimensions_mm

Effective `(width_mm, height_mm)` honouring orientation. Custom

## Signature

```rust
pub fn dimensions_mm(self, orientation: Orientation) -> (f64, f64)
```

## Visibility

- `pub`

## Docstring

Effective `(width_mm, height_mm)` honouring orientation. Custom
sizes are not rotated (user supplied them as-is).

## Source
Lines 87–94 in `crates/oxide-output/src/pdf/page.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [page](/crates/oxide-output/src/pdf/page.md) |
