---
okf_version: "0.2"
type: Function
title: add_text_frame
description: "Append a framed silk string. `content` starts empty; the user edits it"
resource: crates/oxide-app/src/library/editor/footprint/text_frame.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/text_frame/add_text_frame
language: rust
---

# add_text_frame

Append a framed silk string. `content` starts empty; the user edits it

## Signature

```rust
pub fn add_text_frame(fp: &mut Footprint, x_mm: f64, y_mm: f64, w_mm: f64, h_mm: f64)
```

## Visibility

- `pub`

## Docstring

Append a framed silk string. `content` starts empty; the user edits it
via the existing text-edit flow after placement. `size` / `stroke_width`
match the `FootprintAddText` (Place String) defaults so a framed string
renders identically to an unframed one at the same font size.

## Source
Lines 11–22 in `crates/oxide-app/src/library/editor/footprint/text_frame.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text_frame](/crates/oxide-app/src/library/editor/footprint/text_frame.md) |
