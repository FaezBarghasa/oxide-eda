---
okf_version: "0.2"
type: Function
title: detect_duplicate_uuids
description: Report every pair of sheets that share a schematic uuid — copy-as-template
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod/detect_duplicate_uuids
language: rust
---

# detect_duplicate_uuids

Report every pair of sheets that share a schematic uuid — copy-as-template

## Signature

```rust
fn detect_duplicate_uuids(
    sheets: &HashMap<SheetKey, SchematicSheet>,
    issues: &mut Vec<StitchIssue>,
)
```

## Docstring

Report every pair of sheets that share a schematic uuid — copy-as-template
corruption. Sheet identity is the [`SheetKey`], never the uuid; every sheet
in the graph is checked uniformly, root included, since the root is just
another entry in `sheets`.

## Source
Lines 656–678 in `crates/oxide-net/src/project/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-net/src/project/mod.md) |
| called_by | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
