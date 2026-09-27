---
okf_version: "0.2"
type: Function
title: create_new_project
description: "Create a brand-new `.snxprj` at `path` plus a blank companion"
resource: crates/oxide-app/src/app/handlers/document_files/open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/open/create_new_project_1
language: rust
---

# create_new_project

Create a brand-new `.snxprj` at `path` plus a blank companion

## Signature

```rust
fn create_new_project(
        &mut self,
        project_path: &std::path::Path,
    ) -> Result<iced::Task<Message>>
```

## Docstring

Create a brand-new `.snxprj` at `path` plus a blank companion
`<stem>.snxsch` in the same directory, then load the project and
open the schematic as a tab. The `.snxprj` is written empty —
`parse_project` is directory-driven and ignores file content.

## Source
Lines 51–125 in `crates/oxide-app/src/app/handlers/document_files/open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [open](/crates/oxide-app/src/app/handlers/document_files/open.md) |
| calls | [metadata](/crates/oxide-output/src/substitution/metadata.md) |
| calls | [blank_schematic_sheet](/crates/oxide-app/src/app/handlers/document_files/mod/blank_schematic_sheet.md) |
| calls | [context](/crates/oxide-app/src/app/handlers/menu/export/tests/context.md) |
| calls | [atomic_write](/crates/oxide-app/src/app/dispatch/library/recovery/atomic_write.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
