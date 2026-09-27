---
okf_version: "0.2"
type: Class
title: RgbColor
description: An RGB colour (each channel 0.0-1.0). Groups the three channels that
resource: crates/oxide-output/src/pdf/surface.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/surface/RgbColor
language: rust
---

# RgbColor

An RGB colour (each channel 0.0-1.0). Groups the three channels that

## Signature

```rust
pub struct RgbColor
```

## Visibility

- `pub`

## Docstring

An RGB colour (each channel 0.0-1.0). Groups the three channels that
travel together as a single fill colour — built at the `fill_rect` call
site (currently reserved for the v0.9 template backgrounds/fills) instead
of passed as three bare floats.

## Methods

- `r`
- `g`
- `b`

## Source
Lines 17–21 in `crates/oxide-output/src/pdf/surface.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [surface](/crates/oxide-output/src/pdf/surface.md) |
