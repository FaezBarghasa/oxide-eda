---
okf_version: "0.2"
type: Function
title: build_project_netlist
description: "Build the whole-project [`Netlist`] by stitching `graph.roots` down"
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod/build_project_netlist
language: rust
---

# build_project_netlist

Build the whole-project [`Netlist`] by stitching `graph.roots` down

## Signature

```rust
pub fn build_project_netlist(graph: &ProjectGraph) -> ProjectNetlist
```

## Visibility

- `pub`

## Docstring

Build the whole-project [`Netlist`] by stitching `graph.roots` down
through `graph.resolved`. See the module docs for the stitching rules —
and for the byte-identity contract with `build_netlist` for a single-root,
unresolved graph.

## Source
Lines 228–381 in `crates/oxide-net/src/project/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-net/src/project/mod.md) |
| calls | [detect_duplicate_uuids](/crates/oxide-net/src/project/mod/detect_duplicate_uuids.md) |
| calls | [order_roots](/crates/oxide-net/src/project/root_order/order_roots.md) |
| calls | [visit](/crates/oxide-net/src/project/mod/visit.md) |
| calls | [detect_shared_references](/crates/oxide-net/src/project/mod/detect_shared_references.md) |
| calls | [analyze](/crates/oxide-net/src/project/mod/analyze.md) |
| calls | [bucket_join](/crates/oxide-net/src/project/mod/bucket_join.md) |
| calls | [uf_union](/crates/oxide-library/src/primitive/symbol/chain/uf_union.md) |
| calls | [uf_find](/crates/oxide-library/src/primitive/symbol/chain/uf_find.md) |
| calls | [assemble_net](/crates/oxide-net/src/project/mod/assemble_net.md) |
| calls | [NetId](/crates/oxide-types/src/net/NetId.md) |
| calls | [compare_references](/crates/oxide-types/src/designator/compare_references.md) |
| called_by | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
| called_by | [refresh_project_netlist](/crates/oxide-app/src/app/mutation_gateway/refresh_project_netlist.md) |
| called_by | [same_filename_children_of_different_parents_stitch_from_their_own_files](/crates/oxide-net/src/project/tests/same_filename_children_of_different_parents_stitch_from_their_own_files.md) |
| called_by | [stitch_pages](/crates/oxide-net/src/project/tests/stitch_pages.md) |
