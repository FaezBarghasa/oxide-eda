---
okf_version: "0.2"
type: Function
title: gather_refs
description: "Walk an AST and append every distinct `Ref(name)` name to `out`."
resource: crates/oxide-sketch/src/parameter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/parameter/gather_refs
language: rust
---

# gather_refs

Walk an AST and append every distinct `Ref(name)` name to `out`.

## Signature

```rust
fn gather_refs(node: &ExprNode, out: &mut Vec<String>)
```

## Docstring

Walk an AST and append every distinct `Ref(name)` name to `out`.

## Source
Lines 200–232 in `crates/oxide-sketch/src/parameter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameter](/crates/oxide-sketch/src/parameter.md) |
| calls | [rec](/crates/oxide-sketch/src/parameter/rec.md) |
| called_by | [collect_deps](/crates/oxide-sketch/src/parameter/collect_deps.md) |
