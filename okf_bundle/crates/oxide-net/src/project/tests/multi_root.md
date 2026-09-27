---
okf_version: "0.2"
type: Module
title: multi_root
description: "#430 — multi-root / flat-stitch traversal: every declared page the root's"
resource: crates/oxide-net/src/project/tests/multi_root.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/multi_root
language: rust
---

# multi_root

#430 — multi-root / flat-stitch traversal: every declared page the root's

## Docstring

#430 — multi-root / flat-stitch traversal: every declared page the root's
hierarchy never reaches is walked as its own independent top-level page.
Split out of the parent module only to keep that file under the size cap.

The fixture throughout is `Add Existing Sheet`'s routine flat topology:
several sibling pages, none referencing any other, handed to
[`build_project_netlist`](super::super::build_project_netlist) as extra
[`ProjectRoot`](super::super::ProjectRoot)s the root never points at.

Under #466 the page list is the *caller's* to state — `assemble_project_sheets`
computes `pages_outside_the_hierarchy` as "declared, and not reachable
**from the root**" — so these tests pass the pages explicitly via
`stitch_pages` rather than leaving the stitcher to guess which sheets are
orphans. Test 5 is the case that makes the traversal's visited set
load-bearing: a page that another page reaches is still on the caller's
list, and must contribute exactly one occurrence.

## Relationships

| Type | Target |
|------|--------|
| related | [flat_sibling_with_shared_global_label_merges_into_one_net](/crates/oxide-net/src/project/tests/multi_root/flat_sibling_with_shared_global_label_merges_into_one_net.md) |
| related | [two_flat_siblings_merge_by_shared_power_label_root_stays_separate](/crates/oxide-net/src/project/tests/multi_root/two_flat_siblings_merge_by_shared_power_label_root_stays_separate.md) |
| related | [flat_siblings_with_distinct_local_labels_stay_separate](/crates/oxide-net/src/project/tests/multi_root/flat_siblings_with_distinct_local_labels_stay_separate.md) |
| related | [flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge](/crates/oxide-net/src/project/tests/multi_root/flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge.md) |
| related | [a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root](/crates/oxide-net/src/project/tests/multi_root/a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root.md) |
| related | [a_flat_pages_missing_child_is_reported](/crates/oxide-net/src/project/tests/multi_root/a_flat_pages_missing_child_is_reported.md) |
| related | [two_flat_pages_referencing_each_other_is_a_cycle_not_a_hang](/crates/oxide-net/src/project/tests/multi_root/two_flat_pages_referencing_each_other_is_a_cycle_not_a_hang.md) |
| related | [flat_page_traversal_is_deterministic_across_map_insertion_order](/crates/oxide-net/src/project/tests/multi_root/flat_page_traversal_is_deterministic_across_map_insertion_order.md) |
| related | [a_page_listed_twice_still_contributes_one_occurrence](/crates/oxide-net/src/project/tests/multi_root/a_page_listed_twice_still_contributes_one_occurrence.md) |
| related | [stitch_referencing_page_last](/crates/oxide-net/src/project/tests/multi_root/stitch_referencing_page_last.md) |
| related | [a_page_referenced_by_a_later_sorting_page_is_still_stitched_once](/crates/oxide-net/src/project/tests/multi_root/a_page_referenced_by_a_later_sorting_page_is_still_stitched_once.md) |
| related | [page_order_does_not_change_a_referenced_pages_occurrence_count](/crates/oxide-net/src/project/tests/multi_root/page_order_does_not_change_a_referenced_pages_occurrence_count.md) |
| related | [a_page_referencing_the_project_root_does_not_stitch_the_root_twice](/crates/oxide-net/src/project/tests/multi_root/a_page_referencing_the_project_root_does_not_stitch_the_root_twice.md) |
