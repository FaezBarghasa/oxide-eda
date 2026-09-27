---
okf_version: "0.2"
type: Function
title: symbol_context_menu_overlay
description: Right-click context menu overlay for the symbol canvas. Mirrors
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/symbol_context_menu_overlay_1
language: rust
---

# symbol_context_menu_overlay

Right-click context menu overlay for the symbol canvas. Mirrors

## Signature

```rust
pub(in crate::app::view) fn symbol_context_menu_overlay(&self) -> Vec<Element<'_, Message>>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Right-click context menu overlay for the symbol canvas. Mirrors
[`Self::footprint_context_menu_overlay`] 1:1 in structure — see
that method for the coordinate / clamping rationale.

## Source
Lines 472–512 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
