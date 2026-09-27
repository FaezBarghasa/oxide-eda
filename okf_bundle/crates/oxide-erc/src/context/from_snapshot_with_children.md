---
okf_version: "0.2"
type: Function
title: from_snapshot_with_children
description: "`resolved` is THIS sheet's own resolution submap — `cs.filename ->"
resource: crates/oxide-erc/src/context.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/context/from_snapshot_with_children
language: rust
---

# from_snapshot_with_children

`resolved` is THIS sheet's own resolution submap — `cs.filename ->

## Signature

```rust
impl ErcContext { pub fn from_snapshot_with_children(
        snapshot: &SchematicSheet,
        resolved: &HashMap<String, oxide_net::SheetKey>,
        sheets: &HashMap<oxide_net::SheetKey, SchematicSheet>,
    ) -> Self }
```

## Visibility

- `pub`

## Docstring

`resolved` is THIS sheet's own resolution submap — `cs.filename ->
SheetKey` — not a project-wide map; `sheets` is the shared
`SheetKey -> SchematicSheet` table every sheet's submap is looked up
against. Keying `resolved` per-sheet (rather than a single project-wide
filename map) is what keeps this in step with
`oxide_net::build_project_netlist` when two parents in different
directories reference a child by the same filename string (#466).

## Source
Lines 206–220 in `crates/oxide-erc/src/context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context](/crates/oxide-erc/src/context.md) |
