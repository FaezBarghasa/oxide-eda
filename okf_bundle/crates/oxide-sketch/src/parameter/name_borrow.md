---
okf_version: "0.2"
type: Function
title: name_borrow
description: "Borrow a `&str` view of `name` whose lifetime matches `asts`'s"
resource: crates/oxide-sketch/src/parameter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/parameter/name_borrow
language: rust
---

# name_borrow

Borrow a `&str` view of `name` whose lifetime matches `asts`'s

## Signature

```rust
fn name_borrow(name: &str, asts: &'a BTreeMap<String, ExprNode>) -> &'a str
```

## Type Parameters

- `'a`

## Docstring

Borrow a `&str` view of `name` whose lifetime matches `asts`'s
keys, so we can use it as a `HashMap<&str, _>` key.

## Source
Lines 184–188 in `crates/oxide-sketch/src/parameter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameter](/crates/oxide-sketch/src/parameter.md) |
| called_by | [topo_sort](/crates/oxide-sketch/src/parameter/topo_sort.md) |
