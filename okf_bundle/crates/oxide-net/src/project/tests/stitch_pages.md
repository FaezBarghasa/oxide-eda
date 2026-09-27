---
okf_version: "0.2"
type: Function
title: stitch_pages
description: "[`stitch`] plus the flat-page roots #430 added: `pages` names further"
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/stitch_pages
language: rust
---

# stitch_pages

[`stitch`] plus the flat-page roots #430 added: `pages` names further

## Signature

```rust
fn stitch_pages(
    root: &SchematicSheet,
    root_key: &str,
    children: &HashMap<String, SchematicSheet>,
    pages: &[&str],
) -> ProjectNetlist
```

## Docstring

[`stitch`] plus the flat-page roots #430 added: `pages` names further
`children` entries to walk as their own top-level pages, in the order
given. `stitch` is this with no extra pages.

Pages are seeded with `name: None`, matching what #430 ships and what the
app passes: a flat page is a *peer* of the root, so its labels stay
unqualified — `VCC` on page two is the same `VCC` as on the root, which is
the whole point of stitching the pages into one netlist.
[`name_seeded_page_qualifies_its_own_labels`] covers the other setting.

## Source
Lines 230–264 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| calls | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
| called_by | [a_flat_pages_missing_child_is_reported](/crates/oxide-net/src/project/tests/multi_root/a_flat_pages_missing_child_is_reported.md) |
| called_by | [a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root](/crates/oxide-net/src/project/tests/multi_root/a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root.md) |
| called_by | [a_page_listed_twice_still_contributes_one_occurrence](/crates/oxide-net/src/project/tests/multi_root/a_page_listed_twice_still_contributes_one_occurrence.md) |
| called_by | [a_page_referencing_the_project_root_does_not_stitch_the_root_twice](/crates/oxide-net/src/project/tests/multi_root/a_page_referencing_the_project_root_does_not_stitch_the_root_twice.md) |
| called_by | [flat_page_traversal_is_deterministic_across_map_insertion_order](/crates/oxide-net/src/project/tests/multi_root/flat_page_traversal_is_deterministic_across_map_insertion_order.md) |
| called_by | [flat_sibling_with_shared_global_label_merges_into_one_net](/crates/oxide-net/src/project/tests/multi_root/flat_sibling_with_shared_global_label_merges_into_one_net.md) |
| called_by | [flat_siblings_with_distinct_local_labels_stay_separate](/crates/oxide-net/src/project/tests/multi_root/flat_siblings_with_distinct_local_labels_stay_separate.md) |
| called_by | [flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge](/crates/oxide-net/src/project/tests/multi_root/flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge.md) |
| called_by | [stitch_referencing_page_last](/crates/oxide-net/src/project/tests/multi_root/stitch_referencing_page_last.md) |
| called_by | [two_flat_pages_referencing_each_other_is_a_cycle_not_a_hang](/crates/oxide-net/src/project/tests/multi_root/two_flat_pages_referencing_each_other_is_a_cycle_not_a_hang.md) |
| called_by | [two_flat_siblings_merge_by_shared_power_label_root_stays_separate](/crates/oxide-net/src/project/tests/multi_root/two_flat_siblings_merge_by_shared_power_label_root_stays_separate.md) |
| called_by | [stitch](/crates/oxide-net/src/project/tests/stitch.md) |
