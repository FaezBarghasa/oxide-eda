---
okf_version: "0.2"
type: Function
title: canvas_colors_for
description: "The active canvas colour set, derived from the saved theme."
resource: crates/oxide-app/src/app/runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/mod/canvas_colors_for
language: rust
---

# canvas_colors_for

The active canvas colour set, derived from the saved theme.

## Signature

```rust
impl Oxide { pub(crate) fn canvas_colors_for(&self, id: ThemeId) -> oxide_types::theme::CanvasColors }
```

## Visibility

- `pub(crate)`

## Docstring

The active canvas colour set, derived from the saved theme.

One derivation for both readers — `update_canvas_theme` (which
still pushes them into the PCB canvas) and
[`Self::canvas_view_prefs`] (which hands them to the schematic
`Program` each frame). Two copies of this `if` was how the two
could disagree.
Canvas colours for a given theme id. `Custom` reads the loaded
custom theme and falls back to Oxide when none is loaded.

## Source
Lines 200–210 in `crates/oxide-app/src/app/runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [runtime](/crates/oxide-app/src/app/runtime/mod.md) |
| calls | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |
