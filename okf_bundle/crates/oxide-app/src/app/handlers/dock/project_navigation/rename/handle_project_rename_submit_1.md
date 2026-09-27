---
okf_version: "0.2"
type: Function
title: handle_project_rename_submit
description: "Project-root rename — `state.target_path` is the project's"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/rename.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/rename/handle_project_rename_submit_1
language: rust
---

# handle_project_rename_submit

Project-root rename — `state.target_path` is the project's

## Signature

```rust
fn handle_project_rename_submit(
        &mut self,
        state: &crate::app::RenameDialogState,
    ) -> iced::Task<Message>
```

## Docstring

Project-root rename — `state.target_path` is the project's
`.snxprj`; the buffer is the new *stem*. We rename ONLY the
`.snxprj` file and update the project's in-memory `name`.
Companion schematic / pcb files keep their existing filenames —
they're independent entities, possibly shared across workflows
or referenced from version control with their original names.
The `.snxprj`'s `schematic_root` / `pcb_file` are filename
strings that continue to point at the unchanged sheet files.

## Source
Lines 165–243 in `crates/oxide-app/src/app/handlers/dock/project_navigation/rename.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rename](/crates/oxide-app/src/app/handlers/dock/project_navigation/rename.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
