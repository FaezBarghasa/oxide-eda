---
okf_version: "0.2"
type: Function
title: visit
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod/visit
language: rust
---

# visit

## Signature

```rust
fn visit(
    graph: &ProjectGraph<'a>,
    key: &SheetKey,
    sheet: &'a SchematicSheet,
    name_chain: Vec<String>,
    path: &mut Vec<SheetKey>,
    occs: &mut Vec<Occ<'a>>,
    edges: &mut Vec<(usize, usize, usize)>,
    visited: &mut HashSet<SheetKey>,
    issues: &mut Vec<StitchIssue>,
) -> usize
```

## Type Parameters

- `'a`

## Decorators

- `expect(
    clippy::too_many_arguments,
    reason = "9 arguments: the hierarchy DFS threads its accumulators explicitly so the caller can inspect them (#430)"
)`

## Source
Lines 535–604 in `crates/oxide-net/src/project/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-net/src/project/mod.md) |
| called_by | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
