---
okf_version: "0.2"
type: Function
title: bbox
description: Bounding box around every visible symbol entity.
resource: crates/oxide-app/src/library/editor/symbol/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/mod/bbox_1
language: rust
---

# bbox

Bounding box around every visible symbol entity.

## Signature

```rust
pub(crate) fn bbox(&self) -> (f64, f64, f64, f64)
```

## Visibility

- `pub(crate)`

## Docstring

Bounding box around every visible symbol entity.

If the symbol has no pins and no graphics, return a tiny box
around world origin so Fit keeps the origin marker centered.

## Source
Lines 222–296 in `crates/oxide-app/src/library/editor/symbol/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/symbol/canvas/mod.md) |
