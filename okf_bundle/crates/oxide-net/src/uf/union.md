---
okf_version: "0.2"
type: Function
title: union
description: "Union the two equivalence classes containing `a` and `b`."
resource: crates/oxide-net/src/uf.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/uf/union
language: rust
---

# union

Union the two equivalence classes containing `a` and `b`.

## Signature

```rust
pub fn union(parent: &mut HashMap<K, K>, a: K, b: K)
```

## Type Parameters

- `K: Eq + Hash + Copy + Ord`

## Visibility

- `pub`

## Docstring

Union the two equivalence classes containing `a` and `b`.

The surviving representative is always the **smaller** of the two roots, so
a class's representative is the minimum element of that class — a pure
function of the partition, independent of the order the unions were applied
in. Making `rb` the representative (the obvious "point a at b") instead
leaks union order into the root, and `build_netlist` numbers nets by sorted
root, so reversing document wire order could permute `NetId`s and `N$k`
names for an otherwise identical partition (issue #402).

## Source
Lines 56–62 in `crates/oxide-net/src/uf.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [uf](/crates/oxide-net/src/uf.md) |
| calls | [find](/crates/oxide-net/src/uf/find.md) |
