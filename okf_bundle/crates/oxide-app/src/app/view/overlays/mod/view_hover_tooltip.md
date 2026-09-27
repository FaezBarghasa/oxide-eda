---
okf_version: "0.2"
type: Function
title: view_hover_tooltip
description: "Hover tooltip card showing the placed symbol's designator,"
resource: crates/oxide-app/src/app/view/overlays/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/mod/view_hover_tooltip
language: rust
---

# view_hover_tooltip

Hover tooltip card showing the placed symbol's designator,

## Signature

```rust
impl Oxide { pub(super) fn view_hover_tooltip(&self) -> Option<Element<'_, Message>> }
```

## Visibility

- `pub(super)`

## Docstring

Hover tooltip card showing the placed symbol's designator,
value, footprint, and library id. Only paints after the cursor
has dwelled on a Symbol hit for >= 250 ms — gates impulsive
motion from popping the card. Returns None when the gate
hasn't tripped, when no schematic is active, or when the
uuid no longer resolves (e.g. the symbol was deleted while
the dwell timer was running).

## Source
Lines 24–137 in `crates/oxide-app/src/app/view/overlays/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/app/view/overlays/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [iced_font_for_family](/crates/oxide-app/src/fonts/mod/iced_font_for_family.md) |
