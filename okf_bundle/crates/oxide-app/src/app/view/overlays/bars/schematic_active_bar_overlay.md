---
okf_version: "0.2"
type: Function
title: schematic_active_bar_overlay
description: "Schematic Active Bar overlay — only painted on the main window,"
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/schematic_active_bar_overlay
language: rust
---

# schematic_active_bar_overlay

Schematic Active Bar overlay — only painted on the main window,

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn schematic_active_bar_overlay(
        &self,
    ) -> Option<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Schematic Active Bar overlay — only painted on the main window,
so the main canvas's selection set is the right gate.

## Source
Lines 194–226 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
| calls | [view_bar](/crates/oxide-app/src/active_bar/mod/view_bar.md) |
