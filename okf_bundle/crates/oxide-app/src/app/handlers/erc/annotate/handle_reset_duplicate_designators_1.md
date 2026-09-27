---
okf_version: "0.2"
type: Function
title: handle_reset_duplicate_designators
description: "Altium's \"Reset Duplicate Designators\" — find references that"
resource: crates/oxide-app/src/app/handlers/erc/annotate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/erc/annotate/handle_reset_duplicate_designators_1
language: rust
---

# handle_reset_duplicate_designators

Altium's "Reset Duplicate Designators" — find references that

## Signature

```rust
pub(crate) fn handle_reset_duplicate_designators(&mut self) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Altium's "Reset Duplicate Designators" — find references that
appear on more than one symbol across the WHOLE project, reset
just those to `{prefix}?`. Everything else keeps its current
value. The project is the one assembler's answer
([`crate::app::project_sheets::assemble_project_sheets`]): the
declared pages plus everything reachable down the child-sheet
graph. Unopened sheets are re-saved through the native
`.snxsch` writer so the fix is project-wide.

## Source
Lines 198–398 in `crates/oxide-app/src/app/handlers/erc/annotate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotate](/crates/oxide-app/src/app/handlers/erc/annotate.md) |
| calls | [assemble_active_project_sheets](/crates/oxide-app/src/app/project_sheets/assemble_active_project_sheets.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
