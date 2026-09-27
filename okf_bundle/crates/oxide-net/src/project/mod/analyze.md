---
okf_version: "0.2"
type: Function
title: analyze
description: "Level-1 analysis for one sheet: the per-sheet derivation plus sheet-pin"
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod/analyze
language: rust
---

# analyze

Level-1 analysis for one sheet: the per-sheet derivation plus sheet-pin

## Signature

```rust
fn analyze(sheet: &SchematicSheet) -> Analysis<'_>
```

## Docstring

Level-1 analysis for one sheet: the per-sheet derivation plus sheet-pin
anchoring, sampled into the tables the stitcher reads.

## Source
Lines 472–524 in `crates/oxide-net/src/project/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-net/src/project/mod.md) |
| calls | [pt_key](/crates/oxide-erc/src/context/pt_key.md) |
| calls | [uf_find](/crates/oxide-library/src/primitive/symbol/chain/uf_find.md) |
| called_by | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
