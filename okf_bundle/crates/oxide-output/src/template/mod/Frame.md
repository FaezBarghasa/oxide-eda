---
okf_version: "0.2"
type: Class
title: Frame
description: Page frame — the border drawn around the schematic area plus optional
resource: crates/oxide-output/src/template/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/template/mod/Frame
language: rust
---

# Frame

Page frame — the border drawn around the schematic area plus optional

## Signature

```rust
pub struct Frame
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq)`

## Visibility

- `pub`

## Docstring

Page frame — the border drawn around the schematic area plus optional
zone markers (A/B/C… letters and 1/2/3… digits around the edge).
[derive(Debug, Clone, Copy, PartialEq)]

## Methods

- `border_margin_mm`
- `show_zone_markers`
- `horizontal_zones`
- `vertical_zones`

## Source
Lines 54–60 in `crates/oxide-output/src/template/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [template](/crates/oxide-output/src/template/mod.md) |
