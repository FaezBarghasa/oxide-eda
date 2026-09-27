---
okf_version: "0.2"
type: Function
title: format_coord
description: Format floating-point coordinate (mm) to integer Gerber format (e.g. 4.6 format).
resource: crates/oxide-output/src/gerber/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:02:33Z"
concept_id: crates/oxide-output/src/gerber/mod/format_coord_1
language: rust
---

# format_coord

Format floating-point coordinate (mm) to integer Gerber format (e.g. 4.6 format).

## Signature

```rust
fn format_coord(&self, x_mm: f64, y_mm: f64) -> (i64, i64)
```

## Docstring

Format floating-point coordinate (mm) to integer Gerber format (e.g. 4.6 format).

## Source
Lines 433–438 in `crates/oxide-output/src/gerber/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gerber](/crates/oxide-output/src/gerber/mod.md) |
