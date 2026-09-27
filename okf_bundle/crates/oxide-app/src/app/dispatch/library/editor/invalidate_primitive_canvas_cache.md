---
okf_version: "0.2"
type: Function
title: invalidate_primitive_canvas_cache
description: Clear the canvas cache for the primitive editor tab keyed by
resource: crates/oxide-app/src/app/dispatch/library/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/invalidate_primitive_canvas_cache
language: rust
---

# invalidate_primitive_canvas_cache

Clear the canvas cache for the primitive editor tab keyed by

## Signature

```rust
impl Oxide { fn invalidate_primitive_canvas_cache(&mut self, path: &std::path::Path) }
```

## Docstring

Clear the canvas cache for the primitive editor tab keyed by
`path`. Used by the per-library display-settings handlers so
the visible canvas redraws as soon as the user flips bg /
grid / etc.

## Source
Lines 379–386 in `crates/oxide-app/src/app/dispatch/library/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/app/dispatch/library/editor.md) |
