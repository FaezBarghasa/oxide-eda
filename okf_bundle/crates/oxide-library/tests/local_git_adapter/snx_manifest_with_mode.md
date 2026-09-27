---
okf_version: "0.2"
type: Function
title: snx_manifest_with_mode
description: "Build an `.snxlib` manifest in the given workflow mode. Cascade"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/snx_manifest_with_mode
language: rust
---

# snx_manifest_with_mode

Build an `.snxlib` manifest in the given workflow mode. Cascade

## Signature

```rust
fn snx_manifest_with_mode(name: &str, mode: WorkflowMode) -> SnxlibManifest
```

## Docstring

Build an `.snxlib` manifest in the given workflow mode. Cascade
behaviour gates on this — Personal silently bumps every bound row,
Team leaves released rows stale.

## Source
Lines 897–913 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| called_by | [cascade_footprint_personal_mode_auto_bumps_bound_row](/crates/oxide-library/tests/local_git_adapter/cascade_footprint_personal_mode_auto_bumps_bound_row.md) |
| called_by | [cascade_personal_mode_auto_bumps_bound_row](/crates/oxide-library/tests/local_git_adapter/cascade_personal_mode_auto_bumps_bound_row.md) |
| called_by | [cascade_team_mode_leaves_released_row_stale](/crates/oxide-library/tests/local_git_adapter/cascade_team_mode_leaves_released_row_stale.md) |
| called_by | [cascade_team_mode_unreleased_row_auto_bumps](/crates/oxide-library/tests/local_git_adapter/cascade_team_mode_unreleased_row_auto_bumps.md) |
