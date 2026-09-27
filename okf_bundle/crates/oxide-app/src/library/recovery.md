---
okf_version: "0.2"
type: Module
title: recovery
description: "Library recovery dialogs (`v0.9-snxlib-as-file-plan.md` §2 Stage H)."
resource: crates/oxide-app/src/library/recovery.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/recovery
language: rust
---

# recovery

Library recovery dialogs (`v0.9-snxlib-as-file-plan.md` §2 Stage H).

## Docstring

Library recovery dialogs (`v0.9-snxlib-as-file-plan.md` §2 Stage H).

When [`oxide_library::LocalGitAdapter::open`] fails with a
recoverable error — the `.snxlib` file went missing, the `.git/`
directory was deleted, or a row's primitive binding points at a
file that's no longer on disk — the dispatcher routes the error
into one of the three modal flows defined here instead of merely
logging-and-silently-failing. The user always gets a clear choice.

Three dialogs:

1. **Library missing** (`.snxlib` is gone) — *Locate…* re-opens the
file picker so the user can point Oxide at a moved library.
*Remove from project* drops the entry from the project's
`[libraries]` list.

2. **Git missing** (`.snxlib` is fine but `.git/` was deleted) —
*Re-init* runs [`oxide_library::LocalGitAdapter::recover_init`]
which `git init`s a fresh repo at the parent directory and
stages the current working tree as a single
"snxlib re-init" commit. Past per-primitive history is lost
(the warning is bold-red on that button). *Skip (read-only)*
walks away — the dispatcher just logs and keeps going.
*Restore from remote* is reserved for when the manifest
records a `[users.<remote>]` URL; v0.9 leaves the button
disabled because no remote field exists yet (Stage 13+).

3. **Broken primitive binding** (a row's `symbol_uuid` /
`footprint_uuid` resolves to a UUID with no on-disk file) —
*Re-bind…* opens the existing primitive picker so the user
can re-attach a real `.snxsym` / `.snxfpt`. *Remove row*
deletes the row entirely from its table.

Wiring lives on [`crate::library::state::LibraryState::recovery`]
(a single `Option<RecoveryDialog>` slot — only one dialog is
visible at a time). The dispatcher routes adapter errors to it;
the view-side overlay flow renders it from
`app/view/mod.rs::collect_overlays`. The overlay predicate in the
same file gates whether `collect_overlays` runs at all (see the
`[needs_overlay predicate gates modal rendering]` invariant —
without that flag the modal just doesn't paint and clicks
vanish).

## Relationships

| Type | Target |
|------|--------|
| related | [RecoveryDialog](/crates/oxide-app/src/library/recovery/RecoveryDialog.md) |
| related | [LibraryMissingChoice](/crates/oxide-app/src/library/recovery/LibraryMissingChoice.md) |
| related | [GitMissingChoice](/crates/oxide-app/src/library/recovery/GitMissingChoice.md) |
| related | [BrokenBindingChoice](/crates/oxide-app/src/library/recovery/BrokenBindingChoice.md) |
| related | [view](/crates/oxide-app/src/library/recovery/view.md) |
| related | [library_missing_view](/crates/oxide-app/src/library/recovery/library_missing_view.md) |
| related | [git_missing_view](/crates/oxide-app/src/library/recovery/git_missing_view.md) |
| related | [broken_binding_view](/crates/oxide-app/src/library/recovery/broken_binding_view.md) |
| related | [secondary_btn](/crates/oxide-app/src/library/recovery/secondary_btn.md) |
| related | [primary_btn](/crates/oxide-app/src/library/recovery/primary_btn.md) |
| related | [destructive_btn](/crates/oxide-app/src/library/recovery/destructive_btn.md) |
| related | [disabled_btn](/crates/oxide-app/src/library/recovery/disabled_btn.md) |
| related | [close_x](/crates/oxide-app/src/library/recovery/close_x.md) |
| related | [library_missing_carries_path](/crates/oxide-app/src/library/recovery/library_missing_carries_path.md) |
| related | [git_missing_remote_defaults_none](/crates/oxide-app/src/library/recovery/git_missing_remote_defaults_none.md) |
| related | [broken_binding_carries_kind_and_uuids](/crates/oxide-app/src/library/recovery/broken_binding_carries_kind_and_uuids.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
