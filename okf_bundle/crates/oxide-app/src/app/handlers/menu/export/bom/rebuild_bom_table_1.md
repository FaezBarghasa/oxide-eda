---
okf_version: "0.2"
type: Function
title: rebuild_bom_table
description: Build (or rebuild) the BomTable from the current document
resource: crates/oxide-app/src/app/handlers/menu/export/bom.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/bom/rebuild_bom_table_1
language: rust
---

# rebuild_bom_table

Build (or rebuild) the BomTable from the current document

## Signature

```rust
fn rebuild_bom_table(&self, opts: &BomOptions) -> Option<oxide_output::BomTable>
```

## Docstring

Build (or rebuild) the BomTable from the current document
state and the supplied options. Returns `None` when there's
no active schematic to roll up.

## Source
Lines 258–261 in `crates/oxide-app/src/app/handlers/menu/export/bom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-app/src/app/handlers/menu/export/bom.md) |
| calls | [build_export_context](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_context.md) |
| calls | [rollup](/crates/oxide-output/src/bom/mod/rollup.md) |
