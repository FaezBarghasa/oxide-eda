---
okf_version: "0.2"
type: Function
title: pin_hit_by_label
description: "Pin index whose NAME or NUMBER label bounding box contains (x, y),"
resource: crates/oxide-app/src/library/editor/symbol/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/mod/pin_hit_by_label_1
language: rust
---

# pin_hit_by_label

Pin index whose NAME or NUMBER label bounding box contains (x, y),

## Signature

```rust
fn pin_hit_by_label(&self, x: f64, y: f64) -> Option<usize>
```

## Docstring

Pin index whose NAME or NUMBER label bounding box contains (x, y),
respecting the active-unit visibility filter. Lets the user grab a
pin by its text, not just its tip. Iterates in reverse so the
last-drawn pin wins on overlap.

## Source
Lines 191–205 in `crates/oxide-app/src/library/editor/symbol/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/symbol/canvas/mod.md) |
