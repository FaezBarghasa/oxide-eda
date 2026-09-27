---
okf_version: "0.2"
type: Function
title: build_symbol_renderer_snapshot
resource: crates/oxide-app/src/library/editor/symbol/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/mod/build_symbol_renderer_snapshot
language: rust
---

# build_symbol_renderer_snapshot

## Signature

```rust
impl SymbolCanvas<'a> { fn build_symbol_renderer_snapshot(
        &self,
        selected: &Option<SymbolSelection>,
        scale: f32,
    ) -> RendererSnapshot }
```

## Type Parameters

- `'a`

## Source
Lines 517–768 in `crates/oxide-app/src/library/editor/symbol/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/symbol/canvas/mod.md) |
| calls | [graphic_on_part](/crates/oxide-app/src/library/editor/symbol/state/mod/graphic_on_part.md) |
| calls | [is_graphic_selected](/crates/oxide-app/src/library/editor/symbol/canvas/mod/is_graphic_selected.md) |
