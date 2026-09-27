---
okf_version: "0.2"
type: Function
title: handle_bom_preview_open
description: Open the BOM preview modal — Altium parity with Print Preview.
resource: crates/oxide-app/src/app/handlers/menu/export/bom.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/bom/handle_bom_preview_open
language: rust
---

# handle_bom_preview_open

Open the BOM preview modal — Altium parity with Print Preview.

## Signature

```rust
impl Oxide { pub(crate) fn handle_bom_preview_open(&mut self) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Open the BOM preview modal — Altium parity with Print Preview.
Builds the rolled-up table from the active project's
schematic snapshot and seeds the modal with the default
options. The user adjusts grouping / include flags / format
in the modal and clicks Export to drive the file dialog.

## Source
Lines 16–66 in `crates/oxide-app/src/app/handlers/menu/export/bom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-app/src/app/handlers/menu/export/bom.md) |
