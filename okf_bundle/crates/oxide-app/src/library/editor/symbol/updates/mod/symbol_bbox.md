---
okf_version: "0.2"
type: Function
title: symbol_bbox
description: "World-space bbox covering the symbol's body + every pin + every"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/symbol_bbox
language: rust
---

# symbol_bbox

World-space bbox covering the symbol's body + every pin + every

## Signature

```rust
fn symbol_bbox(sym: &oxide_library::Symbol) -> (f64, f64, f64, f64)
```

## Docstring

World-space bbox covering the symbol's body + every pin + every
graphic. Used by `SymbolFit` so the dispatcher can compute a
`Camera::fit_rect` against the active symbol without reaching
into the canvas program. Matches the `SymbolCanvas::bbox` shape
so click-Fit and Home key produce the same viewport.

## Source
Lines 495–570 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| called_by | [apply_symbol_camera](/crates/oxide-app/src/library/editor/symbol/updates/camera/apply_symbol_camera.md) |
