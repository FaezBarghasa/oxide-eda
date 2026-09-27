---
okf_version: "0.2"
type: Function
title: collect_deps
description: "Collect every [`ExprNode::Ref`] name reachable from `name`'s AST."
resource: crates/oxide-sketch/src/parameter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/parameter/collect_deps
language: rust
---

# collect_deps

Collect every [`ExprNode::Ref`] name reachable from `name`'s AST.

## Signature

```rust
fn collect_deps(asts: &BTreeMap<String, ExprNode>, name: &str) -> Vec<String>
```

## Docstring

Collect every [`ExprNode::Ref`] name reachable from `name`'s AST.

## Source
Lines 191–197 in `crates/oxide-sketch/src/parameter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameter](/crates/oxide-sketch/src/parameter.md) |
| calls | [gather_refs](/crates/oxide-sketch/src/parameter/gather_refs.md) |
| called_by | [topo_sort](/crates/oxide-sketch/src/parameter/topo_sort.md) |
