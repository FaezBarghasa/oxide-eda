---
okf_version: "0.2"
type: Function
title: bucket_join
description: "Union `node` into the level-2 class of `name`, seeding the bucket the first"
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod/bucket_join
language: rust
---

# bucket_join

Union `node` into the level-2 class of `name`, seeding the bucket the first

## Signature

```rust
fn bucket_join(l2: &mut HashMap<L2, L2>, bucket: &mut HashMap<String, L2>, name: &str, node: L2)
```

## Docstring

Union `node` into the level-2 class of `name`, seeding the bucket the first
time a name is seen.

## Source
Lines 608–616 in `crates/oxide-net/src/project/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-net/src/project/mod.md) |
| calls | [uf_union](/crates/oxide-library/src/primitive/symbol/chain/uf_union.md) |
| calls | [uf_find](/crates/oxide-library/src/primitive/symbol/chain/uf_find.md) |
| called_by | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
