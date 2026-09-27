---
okf_version: "0.2"
type: Function
title: canvas_view_prefs
description: "Settings the schematic canvas renders with, read fresh from app"
resource: crates/oxide-app/src/app/runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/mod/canvas_view_prefs_1
language: rust
---

# canvas_view_prefs

Settings the schematic canvas renders with, read fresh from app

## Signature

```rust
pub(crate) fn canvas_view_prefs(&self) -> crate::canvas::CanvasViewPrefs<'_>
```

## Visibility

- `pub(crate)`

## Docstring

Settings the schematic canvas renders with, read fresh from app
state on every frame (#631).

Every field here used to be a copy on the canvas struct, pushed by
hand from whichever handler changed it. Most of those sites wrote
the *active* canvas only, so an undocked window kept rendering the
value it was created with — the staleness the issue predicted.
Reading them here means every window draws from one source.

`grid_style` comes from the draft field on purpose: that is the
effective value, carrying the Preferences live preview while the
dialog is open and equal to the committed one otherwise (#630).

## Source
Lines 224–243 in `crates/oxide-app/src/app/runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [runtime](/crates/oxide-app/src/app/runtime/mod.md) |
