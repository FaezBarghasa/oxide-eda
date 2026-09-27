---
okf_version: "0.2"
type: Function
title: uf_union
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/uf_union
language: rust
---

# uf_union

## Signature

```rust
fn uf_union(parent: &mut [usize], a: usize, b: usize)
```

## Source
Lines 541–547 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| calls | [uf_find](/crates/oxide-library/src/primitive/symbol/chain/uf_find.md) |
| called_by | [build_endpoint_clusters](/crates/oxide-library/src/primitive/symbol/chain/build_endpoint_clusters.md) |
| called_by | [validate_topology](/crates/oxide-library/src/primitive/symbol/chain/validate_topology.md) |
| called_by | [bucket_join](/crates/oxide-net/src/project/mod/bucket_join.md) |
| called_by | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
