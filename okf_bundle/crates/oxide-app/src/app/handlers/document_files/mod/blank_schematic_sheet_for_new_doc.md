---
okf_version: "0.2"
type: Function
title: blank_schematic_sheet_for_new_doc
description: "Build the bare-minimum [`SchematicSheet`] used as the starting state"
resource: crates/oxide-app/src/app/handlers/document_files/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/mod/blank_schematic_sheet_for_new_doc
language: rust
---

# blank_schematic_sheet_for_new_doc

Build the bare-minimum [`SchematicSheet`] used as the starting state

## Signature

```rust
pub(crate) fn blank_schematic_sheet_for_new_doc() -> oxide_types::schematic::SchematicSheet
```

## Visibility

- `pub(crate)`

## Docstring

Build the bare-minimum [`SchematicSheet`] used as the starting state
for File ▸ New Project. Only the fields that don't have a serde
default need explicit values; everything else falls through to the
per-field defaults the writer/parser already round-trip.

## Source
Lines 66–68 in `crates/oxide-app/src/app/handlers/document_files/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document_files](/crates/oxide-app/src/app/handlers/document_files/mod.md) |
| calls | [blank_schematic_sheet](/crates/oxide-app/src/app/handlers/document_files/mod/blank_schematic_sheet.md) |
