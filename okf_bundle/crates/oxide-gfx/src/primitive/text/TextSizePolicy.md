---
okf_version: "0.2"
type: Class
title: TextSizePolicy
description: "Per-surface readability limits on rendered text, in **logical** pixels."
resource: crates/oxide-gfx/src/primitive/text.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/text/TextSizePolicy
language: rust
---

# TextSizePolicy

Per-surface readability limits on rendered text, in **logical** pixels.

## Signature

```rust
pub struct TextSizePolicy
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq)`

## Visibility

- `pub`

## Docstring

Per-surface readability limits on rendered text, in **logical** pixels.

Not scene data: the same [`crate::scene::Scene`] is drawn by surfaces that
disagree about how small text may get before it stops being worth drawing
and how large it may grow before it stops being a schematic. The floor and
ceiling are a view decision, so the surface supplies them.

Apply these **before** multiplying by the display's DPI factor — clamping
after would move both thresholds by the scale factor on a HiDPI screen and
silently disagree with a surface that clamped first.
[derive(Clone, Copy, Debug, PartialEq)]

## Methods

- `min_px`
- `max_px`

## Source
Lines 30–33 in `crates/oxide-gfx/src/primitive/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/primitive/text.md) |
