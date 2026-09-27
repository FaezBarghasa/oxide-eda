---
okf_version: "0.2"
type: Function
title: measure_text_advance_runs
resource: crates/oxide-output/src/svg/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/svg/text/measure_text_advance_runs
language: rust
---

# measure_text_advance_runs

## Signature

```rust
fn measure_text_advance_runs(
    face: &Face<'_>,
    runs: &[MarkupRun],
    base_size: f32,
    units_per_em: f32,
) -> f32
```

## Source
Lines 115–136 in `crates/oxide-output/src/svg/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-output/src/svg/text.md) |
| calls | [glyph_advance](/crates/oxide-output/src/svg/text/glyph_advance.md) |
| called_by | [draw_text_outline](/crates/oxide-output/src/svg/text/draw_text_outline.md) |
