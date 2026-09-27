---
okf_version: "0.2"
type: Function
title: view_with_overlay
description: Render the bar + an open dropdown overlay (when one is open).
resource: crates/oxide-widgets/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/active_bar/mod/view_with_overlay
language: rust
---

# view_with_overlay

Render the bar + an open dropdown overlay (when one is open).

## Signature

```rust
pub fn view_with_overlay(
    items: Vec<ActiveBarItem<M>>,
    open_menu: Option<K>,
    close_msg: M,
    entries_for: impl Fn(K) -> Vec<crate::active_bar_dropdown::DropdownEntry<M>>,
    width_hint_for: impl Fn(K) -> Option<f32>,
    tokens: &'a ThemeTokens,
) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M`
- `K`

## Visibility

- `pub`

## Docstring

Render the bar + an open dropdown overlay (when one is open).

`open_menu` indicates which dropdown is currently open (or `None`
when nothing is open). When `Some(key)`, the widget renders the
bar AND a dropdown panel below it AND a transparent backstop
layer behind the bar that fires `close_msg` on any click outside
the bar / panel — Altium-style click-outside-to-dismiss.

`entries_for(key)` is called only when a menu is open and produces
the rows for that menu. `width_hint_for(key)` controls the per-
menu fixed width (None = auto-size from the chip-wrap layout).

Both the trigger button on the bar AND the dropdown items emit
the editor's own message type `M`, so the editor wires its own
state mutations + dispatch arms.

Single-call API: each editor (schematic / footprint / symbol /
upcoming PCB) builds its bar items + a `dropdown_for` closure +
a `close_msg`, then mounts THIS widget directly inside its canvas
Stack. No per-editor overlay-composition code needed.

## Source
Lines 220–276 in `crates/oxide-widgets/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-widgets/src/active_bar/mod.md) |
| calls | [view](/crates/oxide-widgets/src/active_bar/mod/view.md) |
