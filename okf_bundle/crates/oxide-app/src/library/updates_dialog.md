---
okf_version: "0.2"
type: Module
title: updates_dialog
description: "\"Library Updates Available\" modal — Stage 16 of"
resource: crates/oxide-app/src/library/updates_dialog.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/updates_dialog
language: rust
---

# updates_dialog

"Library Updates Available" modal — Stage 16 of

## Docstring

"Library Updates Available" modal — Stage 16 of
`v0.9-snxlib-as-file-plan.md` §3.5.

Surfaces drift between a freshly-opened schematic's placed
`Symbol`s and their source library rows. Each placed Symbol that
went through the `.snxlib` picker carries `library_id`, `row_id`,
and `library_version` (Stage 16's schema additions on
`oxide_types::schematic::Symbol`). On schematic open the
dispatcher walks every Symbol that has a `library_id` set and
compares its pinned version to the row's current version through
the mounted `LibrarySet`. Mismatches accumulate into a
[`LibraryUpdatesState`].

In **Personal** workflow mode (the manifest's
`[workflow] mode = "personal"` default) drift is auto-applied to
every instance silently — no modal opens, just a `tracing::info`
line and a dirty mark on the schematic.

In **Team** workflow mode the dispatcher opens this modal so the
user can review which updates to accept, with checkboxes
pre-toggled by `BumpKind`:

- `Patch` — checked by default (compatible auto-update);
- `Minor` — unchecked by default (review before accepting);
- `Major` — unchecked + ⚠ warning (likely-breaking).

"Update Selected Components" rewrites the picked Symbols to the
latest version + dirty-marks the schematic. "Skip All" leaves
everything pinned and the dispatcher records the schematic in
`LibraryState.skipped_updates_for` so the status bar can show a
persistent indicator until resolved.

The matching `needs_overlay` predicate in
`app/view/mod.rs::view_main_for` includes
`library_updates.is_some()` — without it the modal's `collect_overlays`
contribution is discarded silently and clicks fall through (memory:
`[needs_overlay predicate gates modal rendering]`).

## Relationships

| Type | Target |
|------|--------|
| related | [BumpKind](/crates/oxide-app/src/library/updates_dialog/BumpKind.md) |
| related | [default_checked](/crates/oxide-app/src/library/updates_dialog/default_checked.md) |
| related | [label](/crates/oxide-app/src/library/updates_dialog/label.md) |
| related | [default_checked](/crates/oxide-app/src/library/updates_dialog/default_checked.md) |
| related | [label](/crates/oxide-app/src/library/updates_dialog/label.md) |
| related | [classify_bump](/crates/oxide-app/src/library/updates_dialog/classify_bump.md) |
| related | [parts](/crates/oxide-app/src/library/updates_dialog/parts.md) |
| related | [LibraryUpdateEntry](/crates/oxide-app/src/library/updates_dialog/LibraryUpdateEntry.md) |
| related | [LibraryUpdatesState](/crates/oxide-app/src/library/updates_dialog/LibraryUpdatesState.md) |
| related | [new](/crates/oxide-app/src/library/updates_dialog/new.md) |
| related | [toggle](/crates/oxide-app/src/library/updates_dialog/toggle.md) |
| related | [selected_count](/crates/oxide-app/src/library/updates_dialog/selected_count.md) |
| related | [new](/crates/oxide-app/src/library/updates_dialog/new.md) |
| related | [toggle](/crates/oxide-app/src/library/updates_dialog/toggle.md) |
| related | [selected_count](/crates/oxide-app/src/library/updates_dialog/selected_count.md) |
| related | [view](/crates/oxide-app/src/library/updates_dialog/view.md) |
| related | [render_entry_row](/crates/oxide-app/src/library/updates_dialog/render_entry_row.md) |
| related | [secondary_btn](/crates/oxide-app/src/library/updates_dialog/secondary_btn.md) |
| related | [primary_btn](/crates/oxide-app/src/library/updates_dialog/primary_btn.md) |
| related | [classify_bump_distinguishes_three_buckets](/crates/oxide-app/src/library/updates_dialog/classify_bump_distinguishes_three_buckets.md) |
| related | [default_checked_only_patches](/crates/oxide-app/src/library/updates_dialog/default_checked_only_patches.md) |
| related | [new_sorts_by_ref_des_and_pre_checks_patches](/crates/oxide-app/src/library/updates_dialog/new_sorts_by_ref_des_and_pre_checks_patches.md) |
| related | [toggle_flips_only_matching_uuid](/crates/oxide-app/src/library/updates_dialog/toggle_flips_only_matching_uuid.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
