---
okf_version: "0.2"
type: Class
title: ProjectGraph
description: "Pre-resolved input to [`build_project_netlist`]. The caller (the app) owns"
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod/ProjectGraph
language: rust
---

# ProjectGraph

Pre-resolved input to [`build_project_netlist`]. The caller (the app) owns

## Signature

```rust
pub struct ProjectGraph
```

## Type Parameters

- `'a`

## Decorators

- `derive(Debug, Clone, Copy)`

## Visibility

- `pub`

## Docstring

Pre-resolved input to [`build_project_netlist`]. The caller (the app) owns
every path/host decision — resolving a `ChildSheet.filename` against its
parent's directory, joining, normalizing, case-folding — and hands this
crate opaque keys; this crate only compares them.
[derive(Debug, Clone, Copy)]

## Methods

- `sheets`
- `resolved`
- `roots`

## Source
Lines 110–131 in `crates/oxide-net/src/project/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-net/src/project/mod.md) |
