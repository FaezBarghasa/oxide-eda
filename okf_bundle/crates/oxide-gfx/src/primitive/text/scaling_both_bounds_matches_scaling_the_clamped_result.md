---
okf_version: "0.2"
type: Function
title: scaling_both_bounds_matches_scaling_the_clamped_result
description: "The clamp is in logical pixels, so a surface working in physical"
resource: crates/oxide-gfx/src/primitive/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/text/scaling_both_bounds_matches_scaling_the_clamped_result
language: rust
---

# scaling_both_bounds_matches_scaling_the_clamped_result

The clamp is in logical pixels, so a surface working in physical

## Signature

```rust
fn scaling_both_bounds_matches_scaling_the_clamped_result()
```

## Decorators

- `test`

## Docstring

The clamp is in logical pixels, so a surface working in physical
pixels must scale both bounds rather than the result — the identity
`scene_shader` relies on.
[test]

## Source
Lines 117–125 in `crates/oxide-gfx/src/primitive/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/primitive/text.md) |
| calls | [text_px](/crates/oxide-gfx/src/primitive/text/text_px.md) |
