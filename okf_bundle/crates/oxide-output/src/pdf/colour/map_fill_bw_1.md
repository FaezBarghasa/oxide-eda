---
okf_version: "0.2"
type: Function
title: map_fill_bw
resource: crates/oxide-output/src/pdf/colour.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/colour/map_fill_bw_1
language: rust
---

# map_fill_bw

## Signature

```rust
pub fn map_fill_bw(&self, r: f32, g: f32, b: f32) -> (f32, f32, f32)
```

## Decorators

- `allow(
        dead_code,
        reason = "reserved for the v0.9 fill operations; no caller emits fills yet"
    )`

## Visibility

- `pub`

## Source
Lines 60–69 in `crates/oxide-output/src/pdf/colour.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [colour](/crates/oxide-output/src/pdf/colour.md) |
