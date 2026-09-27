---
okf_version: "0.2"
type: Function
title: row
resource: crates/oxide-app/src/app/view/dialogs/bom/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/bom/table/row
language: rust
---

# row

## Signature

```rust
fn row(references: &[&str], qty: u32, value: &str) -> BomRow
```

## Source
Lines 379–386 in `crates/oxide-app/src/app/view/dialogs/bom/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/app/view/dialogs/bom/table.md) |
| called_by | [keymap_recorder_control](/crates/oxide-app/src/preferences/keymap/keymap_recorder_control.md) |
