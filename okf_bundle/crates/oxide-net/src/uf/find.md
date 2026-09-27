---
okf_version: "0.2"
type: Function
title: find
description: "Find the root of `x`, compressing the path along the way. Iterative"
resource: crates/oxide-net/src/uf.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/uf/find
language: rust
---

# find

Find the root of `x`, compressing the path along the way. Iterative

## Signature

```rust
pub fn find(parent: &mut HashMap<K, K>, x: K) -> K
```

## Type Parameters

- `K: Eq + Hash + Copy`

## Visibility

- `pub`

## Docstring

Find the root of `x`, compressing the path along the way. Iterative
(no recursion) so a degenerate `N`-deep chain doesn't overflow the
thread stack.

Inserts `x` into `parent` if it isn't already present.

## Source
Lines 26–45 in `crates/oxide-net/src/uf.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [uf](/crates/oxide-net/src/uf.md) |
| called_by | [apply_eco](/crates/oxide-net/src/eco/apply_eco.md) |
| called_by | [test_multichannel_netlist_scoping](/crates/oxide-net/src/multichannel/test_multichannel_netlist_scoping.md) |
| called_by | [order_roots](/crates/oxide-net/src/project/root_order/order_roots.md) |
| called_by | [a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root](/crates/oxide-net/src/project/tests/multi_root/a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root.md) |
| called_by | [two_flat_siblings_merge_by_shared_power_label_root_stays_separate](/crates/oxide-net/src/project/tests/multi_root/two_flat_siblings_merge_by_shared_power_label_root_stays_separate.md) |
| called_by | [union](/crates/oxide-net/src/uf/union.md) |
