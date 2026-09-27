---
okf_version: "0.2"
type: Function
title: load_or_activate_project
description: "Append a `LoadedProject` for `project_path` to the workspace if"
resource: crates/oxide-app/src/app/handlers/document_files/open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/open/load_or_activate_project_1
language: rust
---

# load_or_activate_project

Append a `LoadedProject` for `project_path` to the workspace if

## Signature

```rust
fn load_or_activate_project(
        &mut self,
        project_path: &std::path::Path,
    ) -> Result<(crate::app::state::ProjectId, iced::Task<Message>)>
```

## Docstring

Append a `LoadedProject` for `project_path` to the workspace if
it isn't already loaded, then make it active. De-dupes by path
so re-opening the same project just switches activity. Used by
both `open_project_file` (direct .standard_pro open) and the
companion-project path inside `open_schematic_file` /
`open_pcb_file`.

Returns the resolved `ProjectId` **plus** a `Task` that prepares
the project's cold `.snxlib` mounts off the UI thread (#99 part
2c). The `ProjectId` still comes back synchronously because every
caller uses it immediately — `.snxprj` parsing itself stays
synchronous on purpose: `parse_project` measures 0.018 ms and does
not scale with library count, so making it async would buy nothing
and would break that immediate use. The library mounts are the
part that actually costs frames.

## Source
Lines 191–251 in `crates/oxide-app/src/app/handlers/document_files/open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [open](/crates/oxide-app/src/app/handlers/document_files/open.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [parse_project](/crates/oxide-types/src/project/parse_project.md) |
| calls | [auto_mount_project_libraries](/crates/oxide-app/src/library/commands/auto_mount_project_libraries.md) |
| calls | [prepare_mount_off_thread](/crates/oxide-app/src/library/mount/prepare_mount_off_thread.md) |
