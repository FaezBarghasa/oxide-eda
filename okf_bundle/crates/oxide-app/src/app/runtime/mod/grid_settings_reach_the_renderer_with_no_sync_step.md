---
okf_version: "0.2"
type: Function
title: grid_settings_reach_the_renderer_with_no_sync_step
description: "#631 — the canvas used to own copies of these settings, written by"
resource: crates/oxide-app/src/app/runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/mod/grid_settings_reach_the_renderer_with_no_sync_step
language: rust
---

# grid_settings_reach_the_renderer_with_no_sync_step

#631 — the canvas used to own copies of these settings, written by

## Signature

```rust
fn grid_settings_reach_the_renderer_with_no_sync_step()
```

## Decorators

- `test`

## Docstring

#631 — the canvas used to own copies of these settings, written by
whichever handler changed them. Most of those sites wrote
`active_canvas_mut()` only, so an undocked window kept rendering
the value it was created with. `canvas_view_prefs` is now the only
path from `UiState` to the renderer, so a change has to show up
there with nothing else called in between.
[test]

## Source
Lines 278–305 in `crates/oxide-app/src/app/runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [runtime](/crates/oxide-app/src/app/runtime/mod.md) |
