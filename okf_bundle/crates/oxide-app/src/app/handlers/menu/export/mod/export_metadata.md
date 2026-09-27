---
okf_version: "0.2"
type: Function
title: export_metadata
description: "Title-block metadata for the export, plus the project's active variant."
resource: crates/oxide-app/src/app/handlers/menu/export/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/mod/export_metadata
language: rust
---

# export_metadata

Title-block metadata for the export, plus the project's active variant.

## Signature

```rust
fn export_metadata(
    active_engine: &oxide_engine::Engine,
    owning_project: Option<&crate::app::state::LoadedProject>,
) -> ProjectMetadata
```

## Docstring

Title-block metadata for the export, plus the project's active variant.

## Source
Lines 209–230 in `crates/oxide-app/src/app/handlers/menu/export/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [export](/crates/oxide-app/src/app/handlers/menu/export/mod.md) |
| called_by | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
