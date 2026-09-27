---
okf_version: "0.2"
type: Function
title: detect_shared_references
description: Report each reference designator carried by a sheet key instantiated more
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod/detect_shared_references
language: rust
---

# detect_shared_references

Report each reference designator carried by a sheet key instantiated more

## Signature

```rust
fn detect_shared_references(occs: &[Occ], issues: &mut Vec<StitchIssue>)
```

## Docstring

Report each reference designator carried by a sheet key instantiated more
than once: per-occurrence expansion keeps the instances electrically
distinct, but the refdes collide until per-instance annotation exists. One
issue per `(key, reference)`, in sorted-key then document order.

## Source
Lines 622–650 in `crates/oxide-net/src/project/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-net/src/project/mod.md) |
| called_by | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
