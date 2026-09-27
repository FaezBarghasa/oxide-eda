---
okf_version: "0.2"
type: Module
title: library
description: Footprint Library panel handlers — the methods behind the
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library
language: rust
---

# library

Footprint Library panel handlers — the methods behind the

## Docstring

Footprint Library panel handlers — the methods behind the
`FpLibrary*` dock-panel messages that manage the *envelope* of
internal footprints on the active `.snxfpt` editor (open sibling,
select / add / delete / edit / place an internal footprint). The
dispatcher in `mod.rs` routes these panel messages here.

Pure code motion out of the former `sch_library.rs` god-file
(ADR-0001 #163); zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [handle_fp_library_open_sibling](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_open_sibling.md) |
| related | [handle_fp_library_select_internal](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_select_internal.md) |
| related | [handle_fp_library_add_internal](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_add_internal.md) |
| related | [handle_fp_library_delete_internal](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_delete_internal.md) |
| related | [handle_fp_library_edit_internal](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_edit_internal.md) |
| related | [handle_fp_library_open_sibling](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_open_sibling.md) |
| related | [handle_fp_library_select_internal](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_select_internal.md) |
| related | [handle_fp_library_add_internal](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_add_internal.md) |
| related | [handle_fp_library_delete_internal](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_delete_internal.md) |
| related | [handle_fp_library_edit_internal](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_edit_internal.md) |
| related | [open_sibling_returns_the_primitive_open_task](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/open_sibling_returns_the_primitive_open_task.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
