---
okf_version: "0.2"
type: Function
title: names
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/names
language: rust
---

# names

## Signature

```rust
fn names(nl: &Netlist) -> Vec<&str>
```

## Source
Lines 199–201 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| called_by | [duplicate_sibling_name_collision_is_suffixed](/crates/oxide-net/src/project/tests/duplicate_sibling_name_collision_is_suffixed.md) |
| called_by | [local_net_labels_do_not_cross_sheets](/crates/oxide-net/src/project/tests/local_net_labels_do_not_cross_sheets.md) |
| called_by | [flat_siblings_with_distinct_local_labels_stay_separate](/crates/oxide-net/src/project/tests/multi_root/flat_siblings_with_distinct_local_labels_stay_separate.md) |
| called_by | [flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge](/crates/oxide-net/src/project/tests/multi_root/flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge.md) |
| called_by | [same_filename_children_of_different_parents_stitch_from_their_own_files](/crates/oxide-net/src/project/tests/same_filename_children_of_different_parents_stitch_from_their_own_files.md) |
| called_by | [single_sheet_name_collision_matches_build_netlist_and_is_reported](/crates/oxide-net/src/project/tests/single_sheet_name_collision_matches_build_netlist_and_is_reported.md) |
