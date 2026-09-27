---
okf_version: "0.2"
type: Function
title: parse_positive_mm
description: "A dimension buffer is usable only when it reads as a finite, strictly"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/parse_positive_mm
language: rust
---

# parse_positive_mm

A dimension buffer is usable only when it reads as a finite, strictly

## Signature

```rust
fn parse_positive_mm(buffer: &str) -> Option<f64>
```

## Docstring

A dimension buffer is usable only when it reads as a finite, strictly
positive length. Zero, a negative, an infinity and a NaN are all
rejected alongside outright parse failures — each of them would have
produced degenerate geometry.

## Source
Lines 119–124 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.md) |
| called_by | [resolve_tool_dimension_mm](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_tool_dimension_mm.md) |
