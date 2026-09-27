---
okf_version: "0.2"
type: Function
title: apply_dirty_uploads
description: Apply dirty-flag gated uploads and return per-category update counters.
resource: crates/oxide-gfx/src/scene/upload/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T19:56:21Z"
concept_id: crates/oxide-gfx/src/scene/upload/mod/apply_dirty_uploads
language: rust
---

# apply_dirty_uploads

Apply dirty-flag gated uploads and return per-category update counters.

## Signature

```rust
pub fn apply_dirty_uploads(
    scene: &Scene,
    dirty: DirtyFlags,
    target: &mut T,
    text_params: TextUploadParams,
) -> Result<UploadCounters, T::TextError>
```

## Type Parameters

- `T: SceneUploadTarget`

## Visibility

- `pub`

## Docstring

Apply dirty-flag gated uploads and return per-category update counters.

## Source
Lines 287–294 in `crates/oxide-gfx/src/scene/upload/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [upload](/crates/oxide-gfx/src/scene/upload/mod.md) |
| calls | [apply_dirty_uploads_with_culling](/crates/oxide-gfx/src/scene/upload/mod/apply_dirty_uploads_with_culling.md) |
| called_by | [all_dirty_flags_trigger_each_category_once](/crates/oxide-gfx/src/scene/upload/tests/all_dirty_flags_trigger_each_category_once.md) |
| called_by | [dirty_mask_uploads_only_requested_groups](/crates/oxide-gfx/src/scene/upload/tests/dirty_mask_uploads_only_requested_groups.md) |
| called_by | [noop_dirty_mask_skips_all_uploads](/crates/oxide-gfx/src/scene/upload/tests/noop_dirty_mask_skips_all_uploads.md) |
| called_by | [overlay_dirty_uploads_all_overlay_batches](/crates/oxide-gfx/src/scene/upload/tests/overlay_dirty_uploads_all_overlay_batches.md) |
| called_by | [text_upload_error_is_returned](/crates/oxide-gfx/src/scene/upload/tests/text_upload_error_is_returned.md) |
| called_by | [regression_golden_upload_gating_matches_fixture_baseline](/crates/oxide-gfx/tests/regression_golden/regression_golden_upload_gating_matches_fixture_baseline.md) |
