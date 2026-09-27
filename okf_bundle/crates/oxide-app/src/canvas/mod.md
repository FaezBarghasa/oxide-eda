---
okf_version: "0.2"
type: Module
title: canvas
description: Schematic/PCB canvas — wgpu rendering with Altium-style pan/zoom/grid.
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod
language: rust
---

# canvas

Schematic/PCB canvas — wgpu rendering with Altium-style pan/zoom/grid.

## Docstring

Schematic/PCB canvas — wgpu rendering with Altium-style pan/zoom/grid.

Uses `iced::widget::canvas::Program` with a 3-layer cache:
- background: grid, sheet border (cleared on theme/grid/zoom change)
- content: schematic elements (cleared on document edit)
- overlay: selection, cursor, wire-in-progress (cleared every frame)

## Relationships

| Type | Target |
|------|--------|
| related | [CanvasState](/crates/oxide-app/src/canvas/mod/CanvasState.md) |
| related | [CanvasSlot](/crates/oxide-app/src/canvas/mod/CanvasSlot.md) |
| related | [ShapePreviewKind](/crates/oxide-app/src/canvas/mod/ShapePreviewKind.md) |
| related | [ErcMarker](/crates/oxide-app/src/canvas/mod/ErcMarker.md) |
| related | [ErcMarkerSeverity](/crates/oxide-app/src/canvas/mod/ErcMarkerSeverity.md) |
| related | [default](/crates/oxide-app/src/canvas/mod/default.md) |
| related | [default](/crates/oxide-app/src/canvas/mod/default.md) |
| related | [active_render_cache](/crates/oxide-app/src/canvas/mod/active_render_cache.md) |
| related | [active_snapshot](/crates/oxide-app/src/canvas/mod/active_snapshot.md) |
| related | [new](/crates/oxide-app/src/canvas/mod/new.md) |
| related | [clear_overlay_cache](/crates/oxide-app/src/canvas/mod/clear_overlay_cache.md) |
| related | [clear_bg_cache](/crates/oxide-app/src/canvas/mod/clear_bg_cache.md) |
| related | [clear_content_cache](/crates/oxide-app/src/canvas/mod/clear_content_cache.md) |
| related | [live_camera](/crates/oxide-app/src/canvas/mod/live_camera.md) |
| related | [camera](/crates/oxide-app/src/canvas/mod/camera.md) |
| related | [camera_mut](/crates/oxide-app/src/canvas/mod/camera_mut.md) |
| related | [set_render_cache](/crates/oxide-app/src/canvas/mod/set_render_cache.md) |
| related | [fit_to_paper](/crates/oxide-app/src/canvas/mod/fit_to_paper.md) |
| related | [active_render_cache](/crates/oxide-app/src/canvas/mod/active_render_cache.md) |
| related | [active_snapshot](/crates/oxide-app/src/canvas/mod/active_snapshot.md) |
| related | [new](/crates/oxide-app/src/canvas/mod/new.md) |
| related | [clear_overlay_cache](/crates/oxide-app/src/canvas/mod/clear_overlay_cache.md) |
| related | [clear_bg_cache](/crates/oxide-app/src/canvas/mod/clear_bg_cache.md) |
| related | [clear_content_cache](/crates/oxide-app/src/canvas/mod/clear_content_cache.md) |
| related | [live_camera](/crates/oxide-app/src/canvas/mod/live_camera.md) |
| related | [camera](/crates/oxide-app/src/canvas/mod/camera.md) |
| related | [camera_mut](/crates/oxide-app/src/canvas/mod/camera_mut.md) |
| related | [set_render_cache](/crates/oxide-app/src/canvas/mod/set_render_cache.md) |
| related | [fit_to_paper](/crates/oxide-app/src/canvas/mod/fit_to_paper.md) |
| related | [CanvasViewPrefs](/crates/oxide-app/src/canvas/mod/CanvasViewPrefs.md) |
| related | [SchematicCanvas](/crates/oxide-app/src/canvas/mod/SchematicCanvas.md) |
| related | [new](/crates/oxide-app/src/canvas/mod/new.md) |
| related | [auto_focus_set](/crates/oxide-app/src/canvas/mod/auto_focus_set.md) |
| related | [new](/crates/oxide-app/src/canvas/mod/new.md) |
| related | [auto_focus_set](/crates/oxide-app/src/canvas/mod/auto_focus_set.md) |
| related | [deref](/crates/oxide-app/src/canvas/mod/deref.md) |
| related | [deref](/crates/oxide-app/src/canvas/mod/deref.md) |
| related | [update](/crates/oxide-app/src/canvas/mod/update.md) |
| related | [draw](/crates/oxide-app/src/canvas/mod/draw.md) |
| related | [mouse_interaction](/crates/oxide-app/src/canvas/mod/mouse_interaction.md) |
| related | [update](/crates/oxide-app/src/canvas/mod/update.md) |
| related | [draw](/crates/oxide-app/src/canvas/mod/draw.md) |
| related | [mouse_interaction](/crates/oxide-app/src/canvas/mod/mouse_interaction.md) |
| related | [shift_snapshot_for_selection](/crates/oxide-app/src/canvas/mod/shift_snapshot_for_selection.md) |
| related | [active_bar_hit](/crates/oxide-app/src/canvas/mod/active_bar_hit.md) |
| related | [CanvasEvent](/crates/oxide-app/src/canvas/mod/CanvasEvent.md) |
| related | [test_prefs](/crates/oxide-app/src/canvas/mod/test_prefs.md) |
| related | [live_camera_reflects_the_single_source_after_a_mutation](/crates/oxide-app/src/canvas/mod/live_camera_reflects_the_single_source_after_a_mutation.md) |
| related | [a_pending_fit_reaches_the_camera_in_one_hop](/crates/oxide-app/src/canvas/mod/a_pending_fit_reaches_the_camera_in_one_hop.md) |
| related | [a_consumed_fit_does_not_re_apply](/crates/oxide-app/src/canvas/mod/a_consumed_fit_does_not_re_apply.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
