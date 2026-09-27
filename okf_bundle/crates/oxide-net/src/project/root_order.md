---
okf_version: "0.2"
type: Module
title: root_order
description: "Walk order for `ProjectGraph.roots` (#540)."
resource: crates/oxide-net/src/project/root_order.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-net/src/project/root_order
language: rust
---

# root_order

Walk order for `ProjectGraph.roots` (#540).

## Docstring

Walk order for `ProjectGraph.roots` (#540).

The multi-root traversal skips a root it has already reached as an earlier
root's child, so a declared page that another declared page references
contributes one occurrence rather than two. That skip only fires when the
*referencing* page happens to be walked first, and the caller's order —
sorted [`SheetKey`] — is decided by page names, which have nothing to do
with who references whom. Walked the other way round, the referenced page
is stitched twice: its terminals are duplicated, every refdes on it raises
a spurious `SharedReferenceAcrossInstances`, and every subsequent `NetId`
shifts.

[`order_roots`] removes the order-dependence by putting a root that reaches
another root ahead of it, so the skip always has something to skip. The
caller's order survives as the tiebreak, so a flat project — no page
referencing any other, which is what `Add Existing Sheet` produces — is
walked in exactly the order it was handed.

## Relationships

| Type | Target |
|------|--------|
| related | [order_roots](/crates/oxide-net/src/project/root_order/order_roots.md) |
| related | [descendants](/crates/oxide-net/src/project/root_order/descendants.md) |
