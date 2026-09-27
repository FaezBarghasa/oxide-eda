---
okf_version: "0.2"
type: Function
title: assemble_net
description: "Assemble one final net from its level-2 member nodes. Returns `None` when"
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod/assemble_net
language: rust
---

# assemble_net

Assemble one final net from its level-2 member nodes. Returns `None` when

## Signature

```rust
fn assemble_net(mut members: Vec<L2>, occs: &[Occ], analyses: &[Analysis]) -> Option<RawNet>
```

## Docstring

Assemble one final net from its level-2 member nodes. Returns `None` when
the group carries no terminals (a dangling label/pin forms no net).

## Source
Lines 396–468 in `crates/oxide-net/src/project/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-net/src/project/mod.md) |
| called_by | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
