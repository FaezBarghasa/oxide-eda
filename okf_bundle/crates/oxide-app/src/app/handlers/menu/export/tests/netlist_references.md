---
okf_version: "0.2"
type: Function
title: netlist_references
description: Every component reference the exported netlist carries a terminal for —
resource: crates/oxide-app/src/app/handlers/menu/export/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:59Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/tests/netlist_references
language: rust
---

# netlist_references

Every component reference the exported netlist carries a terminal for —

## Signature

```rust
fn netlist_references(ctx: &oxide_output::ExportContext) -> Vec<String>
```

## Docstring

Every component reference the exported netlist carries a terminal for —
what a dropped subtree costs on the board.

## Source
Lines 184–194 in `crates/oxide-app/src/app/handlers/menu/export/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/app/handlers/menu/export/tests.md) |
| calls | [dedup](/crates/oxide-sketch/src/geom/simplify/dedup.md) |
