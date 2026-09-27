---
okf_version: "0.2"
type: Function
title: graphic_kind_to_summary
description: "Project a `SymbolGraphicKind` into a [`GraphicKindSummary`] so the"
resource: crates/oxide-app/src/app/runtime/symbol_ctx.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/symbol_ctx/graphic_kind_to_summary
language: rust
---

# graphic_kind_to_summary

Project a `SymbolGraphicKind` into a [`GraphicKindSummary`] so the

## Signature

```rust
fn graphic_kind_to_summary(
    kind: &oxide_library::SymbolGraphicKind,
) -> crate::panels::GraphicKindSummary
```

## Docstring

Project a `SymbolGraphicKind` into a [`GraphicKindSummary`] so the
Properties panel can render per-shape fields without depending on
the library type.

## Source
Lines 164–206 in `crates/oxide-app/src/app/runtime/symbol_ctx.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_ctx](/crates/oxide-app/src/app/runtime/symbol_ctx.md) |
| called_by | [build_symbol_editor_panel_ctx](/crates/oxide-app/src/app/runtime/symbol_ctx/build_symbol_editor_panel_ctx.md) |
