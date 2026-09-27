---
okf_version: "0.2"
type: Class
title: FootprintContextMenuState
description: "`(x, y)` are **window-absolute** screen coords (already include"
resource: crates/oxide-app/src/library/editor/footprint/state/context_menu.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/context_menu/FootprintContextMenuState
language: rust
---

# FootprintContextMenuState

`(x, y)` are **window-absolute** screen coords (already include

## Signature

```rust
pub struct FootprintContextMenuState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

`(x, y)` are **window-absolute** screen coords (already include
menu-bar + tab-bar offsets). `target` records what the cursor was
over at right-click time so the renderer can pick between the
empty-canvas variant and the on-pad variant. `submenu` tracks
which submenu (if any) is hover-expanded.
[derive(Debug, Clone)]

## Methods

- `x`
- `y`
- `target`
- `submenu`

## Source
Lines 9–14 in `crates/oxide-app/src/library/editor/footprint/state/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/footprint/state/context_menu.md) |
