---
okf_version: "0.2"
type: Function
title: symbol_editor_active_bar_overlay
description: v0.13 — symbol library editor active bar (+ its dropdown overlay)
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/symbol_editor_active_bar_overlay
language: rust
---

# symbol_editor_active_bar_overlay

v0.13 — symbol library editor active bar (+ its dropdown overlay)

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn symbol_editor_active_bar_overlay(
        &self,
    ) -> Vec<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

v0.13 — symbol library editor active bar (+ its dropdown overlay)
mounted at the SAME app-view layer as the schematic / footprint
bars.

## Source
Lines 428–467 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
