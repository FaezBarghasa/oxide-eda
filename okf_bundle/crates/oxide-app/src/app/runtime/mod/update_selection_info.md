---
okf_version: "0.2"
type: Function
title: update_selection_info
description: "Refresh `panel_ctx` selection fields from the active canvas."
resource: crates/oxide-app/src/app/runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/mod/update_selection_info
language: rust
---

# update_selection_info

Refresh `panel_ctx` selection fields from the active canvas.

## Signature

```rust
impl Oxide { pub(crate) fn update_selection_info(&mut self) }
```

## Visibility

- `pub(crate)`

## Docstring

Refresh `panel_ctx` selection fields from the active canvas.

NOTE: `panel_ctx` is shared across every window — the dock
panels, Properties panel, and status bar all read these
fields. When an undocked window handles a canvas event via
the swap trick, "active canvas" refers to the undocked
window's canvas for the duration of the event, so this
function writes THAT window's selection into the shared
panel_ctx. End result: main-window panels reflect the
most-recently-interacted-with window's selection. This is
intentional "last-touched wins" behaviour.

## Source
Lines 115–189 in `crates/oxide-app/src/app/runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [runtime](/crates/oxide-app/src/app/runtime/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
