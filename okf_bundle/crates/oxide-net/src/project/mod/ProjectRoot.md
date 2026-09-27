---
okf_version: "0.2"
type: Class
title: ProjectRoot
description: One entry point the stitcher walks the hierarchy from.
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod/ProjectRoot
language: rust
---

# ProjectRoot

One entry point the stitcher walks the hierarchy from.

## Signature

```rust
pub struct ProjectRoot
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

One entry point the stitcher walks the hierarchy from.

Every root starts with an **empty** name chain, so its qualifiable
(Hierarchical/Net) label names stay bare. For the primary root that is what
keeps the single-root byte-identity contract with `build_netlist`; for a
declared page (#430) it is the deliberate reading of what a page *is* — a
peer of the root rather than something nested under it, so a `VCC` on page
two is the same net as a `VCC` on page one.

A per-root name seed would be the knob for the other reading — qualifying a
page's sheet-scoped labels by its own stem, which changes exported net
names. Nothing asks for it, so it is not carried: add it back with the first
caller that wants it rather than shipping a field only a test ever sets.
[derive(Debug, Clone, PartialEq, Eq)]

## Methods

- `key`

## Source
Lines 101–103 in `crates/oxide-net/src/project/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-net/src/project/mod.md) |
