---
okf_version: "0.2"
type: Function
title: collect_track_items
description: Build the per-row pick-list for the project-scope Enable Version
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/collect_track_items
language: rust
---

# collect_track_items

Build the per-row pick-list for the project-scope Enable Version

## Signature

```rust
pub(crate) fn collect_track_items(
    project: &crate::app::state::LoadedProject,
    project_dir: &std::path::Path,
) -> Vec<crate::app::TrackItem>
```

## Visibility

- `pub(crate)`

## Docstring

Build the per-row pick-list for the project-scope Enable Version
Control modal. Surfaces the `.snxprj`, every sheet, the pcb file,
and each `.snxlib` directory as separately tickable rows so the
user can opt expensive folders out of the initial commit.

## Source
Lines 234–304 in `crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [version_control](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.md) |
| called_by | [open_enable_version_control_dialog](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/open_enable_version_control_dialog.md) |
