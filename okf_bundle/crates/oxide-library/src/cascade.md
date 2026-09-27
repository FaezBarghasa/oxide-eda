---
okf_version: "0.2"
type: Module
title: cascade
description: "Primitive-save cascade engine — Stage 15 of `v0.9-snxlib-as-file-plan.md`."
resource: crates/oxide-library/src/cascade.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/cascade
language: rust
---

# cascade

Primitive-save cascade engine — Stage 15 of `v0.9-snxlib-as-file-plan.md`.

## Docstring

Primitive-save cascade engine — Stage 15 of `v0.9-snxlib-as-file-plan.md`.

When a user saves a Symbol / Footprint / Sim primitive, every
[`ComponentRow`](crate::component::ComponentRow) whose
`<kind>_ref.uuid` matches the primitive UUID has its pinned
`<kind>_version` cell drift away from the new primitive version.
The cascade engine scans those rows and, depending on the library's
[`WorkflowMode`], either auto-bumps them in place or leaves them
flagged as stale bindings.

## Buckets (per plan §3.5.cascade)

- **Auto-cascade-eligible**: a row whose `released == false`. The
row's pinned `<kind>_version` advances to the new primitive
version *and* the row's own `version` patch-bumps so the
schematic-side Library Updates dialog (Stage 16) picks the change
up. Personal mode treats *every* row as eligible — released flags
are hidden in that mode.
- **Needs-review (stale)**: a row whose `released == true` under
`Team` mode. The row's `<kind>_version` cell stays at the old
value; the Library Browser already renders a "stale" indicator
when the binding's pinned version drifts from the bound
primitive's current version.

Stage 15 ships the *data-side* logic only — the Team-mode cascade
modal lives in plan §3.5.cascade and arrives in a v0.9.x polish
pass. The engine is synchronous because the
[`LibraryAdapter`](crate::adapter::LibraryAdapter) trait is
synchronous; making it async would force the whole trait surface
to gain `async fn`. Async cascade is a v1.x concern.

## Relationships

| Type | Target |
|------|--------|
| related | [CascadeReport](/crates/oxide-library/src/cascade/CascadeReport.md) |
| related | [is_empty](/crates/oxide-library/src/cascade/is_empty.md) |
| related | [is_empty](/crates/oxide-library/src/cascade/is_empty.md) |
| related | [patch_bump](/crates/oxide-library/src/cascade/patch_bump.md) |
| related | [cascade_after_symbol_save](/crates/oxide-library/src/cascade/cascade_after_symbol_save.md) |
| related | [cascade_after_footprint_save](/crates/oxide-library/src/cascade/cascade_after_footprint_save.md) |
| related | [cascade_after_sim_save](/crates/oxide-library/src/cascade/cascade_after_sim_save.md) |
| related | [PrimitiveKindTag](/crates/oxide-library/src/cascade/PrimitiveKindTag.md) |
| related | [label](/crates/oxide-library/src/cascade/label.md) |
| related | [label](/crates/oxide-library/src/cascade/label.md) |
| related | [cascade_after_save](/crates/oxide-library/src/cascade/cascade_after_save.md) |
| related | [row_binds_to](/crates/oxide-library/src/cascade/row_binds_to.md) |
| related | [apply_cascade_bump](/crates/oxide-library/src/cascade/apply_cascade_bump.md) |
| related | [cascade_commit_message](/crates/oxide-library/src/cascade/cascade_commit_message.md) |
| related | [short_uuid](/crates/oxide-library/src/cascade/short_uuid.md) |
| related | [patch_bump_increments_clean_semver](/crates/oxide-library/src/cascade/patch_bump_increments_clean_semver.md) |
| related | [patch_bump_falls_back_on_garbage](/crates/oxide-library/src/cascade/patch_bump_falls_back_on_garbage.md) |
| related | [cascade_report_default_is_empty](/crates/oxide-library/src/cascade/cascade_report_default_is_empty.md) |
