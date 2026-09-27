---
okf_version: "0.2"
type: Function
title: apply_dirty_uploads_with_culling
description: Apply dirty-gated uploads with optional viewport culling.
resource: crates/oxide-gfx/src/scene/upload/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T19:56:21Z"
concept_id: crates/oxide-gfx/src/scene/upload/mod/apply_dirty_uploads_with_culling
language: rust
---

# apply_dirty_uploads_with_culling

Apply dirty-gated uploads with optional viewport culling.

## Signature

```rust
pub fn apply_dirty_uploads_with_culling(
    scene: &Scene,
    dirty: DirtyFlags,
    target: &mut T,
    text_params: TextUploadParams,
    culling: UploadCulling,
) -> Result<UploadCounters, T::TextError>
```

## Type Parameters

- `T: SceneUploadTarget`

## Visibility

- `pub`

## Docstring

Apply dirty-gated uploads with optional viewport culling.

## Source
Lines 297–385 in `crates/oxide-gfx/src/scene/upload/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [upload](/crates/oxide-gfx/src/scene/upload/mod.md) |
| calls | [cull_items](/crates/oxide-gfx/src/scene/upload/mod/cull_items.md) |
| calls | [line_envelope](/crates/oxide-gfx/src/scene/upload/mod/line_envelope.md) |
| calls | [circle_envelope](/crates/oxide-gfx/src/scene/upload/mod/circle_envelope.md) |
| calls | [arc_envelope](/crates/oxide-gfx/src/scene/upload/mod/arc_envelope.md) |
| calls | [text_envelope](/crates/oxide-gfx/src/scene/upload/mod/text_envelope.md) |
| called_by | [apply_dirty_uploads](/crates/oxide-gfx/src/scene/upload/mod/apply_dirty_uploads.md) |
| called_by | [disabled_culling_keeps_full_batches](/crates/oxide-gfx/src/scene/upload/tests/disabled_culling_keeps_full_batches.md) |
| called_by | [theme_dirty_can_coexist_with_geometry_updates](/crates/oxide-gfx/src/scene/upload/tests/theme_dirty_can_coexist_with_geometry_updates.md) |
| called_by | [theme_dirty_refreshes_without_geometry_uploads](/crates/oxide-gfx/src/scene/upload/tests/theme_dirty_refreshes_without_geometry_uploads.md) |
| called_by | [viewport_culling_filters_core_primitive_batches](/crates/oxide-gfx/src/scene/upload/tests/viewport_culling_filters_core_primitive_batches.md) |
| called_by | [viewport_culling_filters_overlay_and_erc_batches](/crates/oxide-gfx/src/scene/upload/tests/viewport_culling_filters_overlay_and_erc_batches.md) |
| called_by | [regression_golden_upload_gating_matches_fixture_baseline](/crates/oxide-gfx/tests/regression_golden/regression_golden_upload_gating_matches_fixture_baseline.md) |
