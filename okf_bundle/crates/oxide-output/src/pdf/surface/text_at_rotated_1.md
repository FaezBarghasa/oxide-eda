---
okf_version: "0.2"
type: Function
title: text_at_rotated
description: "Emit rotated text at (x, y) with a text matrix."
resource: crates/oxide-output/src/pdf/surface.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/surface/text_at_rotated_1
language: rust
---

# text_at_rotated

Emit rotated text at (x, y) with a text matrix.

## Signature

```rust
pub fn text_at_rotated(
        &mut self,
        x: f32,
        y: f32,
        font_name: &str,
        size_pt: f32,
        text: &str,
        rotation_deg: f32,
    )
```

## Visibility

- `pub`

## Docstring

Emit rotated text at (x, y) with a text matrix.

## Source
Lines 131–153 in `crates/oxide-output/src/pdf/surface.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [surface](/crates/oxide-output/src/pdf/surface.md) |
| calls | [escape_pdf_string](/crates/oxide-output/src/pdf/surface/escape_pdf_string.md) |
