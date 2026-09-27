---
okf_version: "0.2"
type: Function
title: order_roots
description: "The order to walk `graph.roots` in: every root that reaches another root"
resource: crates/oxide-net/src/project/root_order.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-net/src/project/root_order/order_roots
language: rust
---

# order_roots

The order to walk `graph.roots` in: every root that reaches another root

## Signature

```rust
pub(super) fn order_roots(graph: &ProjectGraph) -> Vec<SheetKey>
```

## Visibility

- `pub(super)`

## Docstring

The order to walk `graph.roots` in: every root that reaches another root
precedes it, ties broken by the caller's order.

Duplicate keys are dropped (first occurrence wins) — a repeated root is
skipped by the traversal anyway, and deduplicating here keeps a doubled
page from being counted as its own referencer.

The project root is not pinned to the front. If some page references it,
the root is walked as that page's child instead of as a top-level page,
which is the same rule every other root gets. The alternative — pin it and
let the referencing page reach it a second time — is the duplication this
module exists to remove, and nothing downstream requires the root to be
occurrence 0.

## Source
Lines 36–78 in `crates/oxide-net/src/project/root_order.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [root_order](/crates/oxide-net/src/project/root_order.md) |
| calls | [descendants](/crates/oxide-net/src/project/root_order/descendants.md) |
| calls | [find](/crates/oxide-net/src/uf/find.md) |
| called_by | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
