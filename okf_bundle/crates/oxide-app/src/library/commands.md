---
okf_version: "0.2"
type: Module
title: commands
description: Library subsystem command helpers.
resource: crates/oxide-app/src/library/commands.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/commands
language: rust
---

# commands

Library subsystem command helpers.

## Docstring

Library subsystem command helpers.

Thin wrappers around `LibraryAdapter` calls that the dispatcher
pulls in — keeping the dispatch file small and the iced glue out
of the library layer. Each helper takes `&mut LibraryState` and
returns a `Result`; non-fatal errors surface via `tracing::warn!`
with structured fields, matching the rest of the codebase.

## Relationships

| Type | Target |
|------|--------|
| related | [open_library](/crates/oxide-app/src/library/commands/open_library.md) |
| related | [PendingLibrarySpec](/crates/oxide-app/src/library/commands/PendingLibrarySpec.md) |
| related | [register_pending_library](/crates/oxide-app/src/library/commands/register_pending_library.md) |
| related | [materialize_pending_library](/crates/oxide-app/src/library/commands/materialize_pending_library.md) |
| related | [create_library_at](/crates/oxide-app/src/library/commands/create_library_at.md) |
| related | [create_library](/crates/oxide-app/src/library/commands/create_library.md) |
| related | [AutoMountOutcome](/crates/oxide-app/src/library/commands/AutoMountOutcome.md) |
| related | [auto_mount_project_libraries](/crates/oxide-app/src/library/commands/auto_mount_project_libraries.md) |
| related | [create_component_row](/crates/oxide-app/src/library/commands/create_component_row.md) |
| related | [list_components_filtered](/crates/oxide-app/src/library/commands/list_components_filtered.md) |
| related | [jump_to_use_site](/crates/oxide-app/src/library/commands/jump_to_use_site.md) |
