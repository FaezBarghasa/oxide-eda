---
okf_version: "0.2"
type: Function
title: descendants
description: "Every key reachable from `start` through resolved child references,"
resource: crates/oxide-net/src/project/root_order.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-net/src/project/root_order/descendants
language: rust
---

# descendants

Every key reachable from `start` through resolved child references,

## Signature

```rust
fn descendants(graph: &ProjectGraph, start: &SheetKey) -> HashSet<SheetKey>
```

## Docstring

Every key reachable from `start` through resolved child references,
excluding `start` itself — a key that only reaches itself through a cycle
is not its own referencer.

## Source
Lines 83–106 in `crates/oxide-net/src/project/root_order.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [root_order](/crates/oxide-net/src/project/root_order.md) |
| called_by | [order_roots](/crates/oxide-net/src/project/root_order/order_roots.md) |
