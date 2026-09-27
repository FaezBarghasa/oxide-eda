---
okf_version: "0.2"
type: Module
title: upload
description: Dirty-flag driven scene upload gating.
resource: crates/oxide-gfx/src/scene/upload/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T19:56:21Z"
concept_id: crates/oxide-gfx/src/scene/upload/mod
language: rust
---

# upload

Dirty-flag driven scene upload gating.

## Docstring

Dirty-flag driven scene upload gating.

CLEAN ROOM DECLARATION
This module was written without reference to GPL-licensed software.
Sources: IPC-2612-1, IEEE 315, IEC 60617, wgpu/WGSL public docs.

## Relationships

| Type | Target |
|------|--------|
| related | [TextUploadParams](/crates/oxide-gfx/src/scene/upload/mod/TextUploadParams.md) |
| related | [new](/crates/oxide-gfx/src/scene/upload/mod/new.md) |
| related | [new](/crates/oxide-gfx/src/scene/upload/mod/new.md) |
| related | [default](/crates/oxide-gfx/src/scene/upload/mod/default.md) |
| related | [default](/crates/oxide-gfx/src/scene/upload/mod/default.md) |
| related | [ViewportAabbMm](/crates/oxide-gfx/src/scene/upload/mod/ViewportAabbMm.md) |
| related | [new](/crates/oxide-gfx/src/scene/upload/mod/new.md) |
| related | [envelope](/crates/oxide-gfx/src/scene/upload/mod/envelope.md) |
| related | [new](/crates/oxide-gfx/src/scene/upload/mod/new.md) |
| related | [envelope](/crates/oxide-gfx/src/scene/upload/mod/envelope.md) |
| related | [UploadCulling](/crates/oxide-gfx/src/scene/upload/mod/UploadCulling.md) |
| related | [disabled](/crates/oxide-gfx/src/scene/upload/mod/disabled.md) |
| related | [viewport](/crates/oxide-gfx/src/scene/upload/mod/viewport.md) |
| related | [envelope](/crates/oxide-gfx/src/scene/upload/mod/envelope.md) |
| related | [disabled](/crates/oxide-gfx/src/scene/upload/mod/disabled.md) |
| related | [viewport](/crates/oxide-gfx/src/scene/upload/mod/viewport.md) |
| related | [envelope](/crates/oxide-gfx/src/scene/upload/mod/envelope.md) |
| related | [default](/crates/oxide-gfx/src/scene/upload/mod/default.md) |
| related | [default](/crates/oxide-gfx/src/scene/upload/mod/default.md) |
| related | [IndexedEnvelope](/crates/oxide-gfx/src/scene/upload/mod/IndexedEnvelope.md) |
| related | [envelope](/crates/oxide-gfx/src/scene/upload/mod/envelope.md) |
| related | [envelope](/crates/oxide-gfx/src/scene/upload/mod/envelope.md) |
| related | [line_envelope](/crates/oxide-gfx/src/scene/upload/mod/line_envelope.md) |
| related | [circle_envelope](/crates/oxide-gfx/src/scene/upload/mod/circle_envelope.md) |
| related | [arc_envelope](/crates/oxide-gfx/src/scene/upload/mod/arc_envelope.md) |
| related | [polygon_envelope](/crates/oxide-gfx/src/scene/upload/mod/polygon_envelope.md) |
| related | [text_envelope](/crates/oxide-gfx/src/scene/upload/mod/text_envelope.md) |
| related | [cull_items](/crates/oxide-gfx/src/scene/upload/mod/cull_items.md) |
| related | [UploadCounters](/crates/oxide-gfx/src/scene/upload/mod/UploadCounters.md) |
| related | [total_updates](/crates/oxide-gfx/src/scene/upload/mod/total_updates.md) |
| related | [geometry_uploads](/crates/oxide-gfx/src/scene/upload/mod/geometry_uploads.md) |
| related | [is_theme_only_refresh](/crates/oxide-gfx/src/scene/upload/mod/is_theme_only_refresh.md) |
| related | [is_idle](/crates/oxide-gfx/src/scene/upload/mod/is_idle.md) |
| related | [total_updates](/crates/oxide-gfx/src/scene/upload/mod/total_updates.md) |
| related | [geometry_uploads](/crates/oxide-gfx/src/scene/upload/mod/geometry_uploads.md) |
| related | [is_theme_only_refresh](/crates/oxide-gfx/src/scene/upload/mod/is_theme_only_refresh.md) |
| related | [is_idle](/crates/oxide-gfx/src/scene/upload/mod/is_idle.md) |
| related | [SceneUploadTarget](/crates/oxide-gfx/src/scene/upload/mod/SceneUploadTarget.md) |
| related | [apply_dirty_uploads](/crates/oxide-gfx/src/scene/upload/mod/apply_dirty_uploads.md) |
| related | [apply_dirty_uploads_with_culling](/crates/oxide-gfx/src/scene/upload/mod/apply_dirty_uploads_with_culling.md) |
| related | [rstar](/_dependencies/cargo/rstar.md) |
