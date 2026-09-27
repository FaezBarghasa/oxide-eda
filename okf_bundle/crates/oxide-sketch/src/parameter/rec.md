---
okf_version: "0.2"
type: Function
title: rec
resource: crates/oxide-sketch/src/parameter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/parameter/rec
language: rust
---

# rec

## Signature

```rust
fn rec(node: &ExprNode, seen: &mut HashSet<String>, out: &mut Vec<String>)
```

## Source
Lines 201–229 in `crates/oxide-sketch/src/parameter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameter](/crates/oxide-sketch/src/parameter.md) |
| called_by | [gather_refs](/crates/oxide-sketch/src/parameter/gather_refs.md) |
