---
okf_version: "0.2"
type: Function
title: rgb
description: "`oxide_types::theme::Color` is u8 RGBA — strip alpha and divide"
resource: crates/oxide-output/src/pdf/palette.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/pdf/palette/rgb
language: rust
---

# rgb

`oxide_types::theme::Color` is u8 RGBA — strip alpha and divide

## Signature

```rust
fn rgb(c: oxide_types::theme::Color) -> (f32, f32, f32)
```

## Docstring

`oxide_types::theme::Color` is u8 RGBA — strip alpha and divide
by 255 so the renderer can feed PDF / tiny-skia f32 colour ops.

## Source
Lines 133–135 in `crates/oxide-output/src/pdf/palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [palette](/crates/oxide-output/src/pdf/palette.md) |
| called_by | [from](/crates/oxide-output/src/pdf/palette/from.md) |
