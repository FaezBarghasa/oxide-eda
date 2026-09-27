# multi_root

## Functions

- [a_flat_pages_missing_child_is_reported](a_flat_pages_missing_child_is_reported.md) — 6 ── A flat page's own missing child is still reported.
- [a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root](a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root.md) — 5 ── THE VISITED-SET CASE. A flat page can itself have its own child sheet
- [a_page_listed_twice_still_contributes_one_occurrence](a_page_listed_twice_still_contributes_one_occurrence.md) — 10 ── #466 × #430: a page listed twice contributes one occurrence, not two.
- [a_page_referenced_by_a_later_sorting_page_is_still_stitched_once](a_page_referenced_by_a_later_sorting_page_is_still_stitched_once.md) — 11 ── #540. Test 5 proved the visited-set skip works when the *referencing*
- [a_page_referencing_the_project_root_does_not_stitch_the_root_twice](a_page_referencing_the_project_root_does_not_stitch_the_root_twice.md) — 13 ── The same rule applied to the project root: a page that references the
- [flat_page_traversal_is_deterministic_across_map_insertion_order](flat_page_traversal_is_deterministic_across_map_insertion_order.md) — 8 ── Determinism: the result must not depend on the `sheets` map's hash
- [flat_sibling_with_shared_global_label_merges_into_one_net](flat_sibling_with_shared_global_label_merges_into_one_net.md) — 1 ── A page nobody references contributes its own net, and a shared
- [flat_siblings_with_distinct_local_labels_stay_separate](flat_siblings_with_distinct_local_labels_stay_separate.md) — 3 ── Local `Net` labels never cross sheets (rule 4) — two flat pages with
- [flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge](flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge.md) — 4 ── Two flat pages that both happen to carry the SAME bare local `Net`
- [page_order_does_not_change_a_referenced_pages_occurrence_count](page_order_does_not_change_a_referenced_pages_occurrence_count.md) — 12 ── ...and the caller's page order does not change the answer. Whichever
- [stitch_referencing_page_last](stitch_referencing_page_last.md) — Test 5's topology with the names swapped: the referenced page is
- [two_flat_pages_referencing_each_other_is_a_cycle_not_a_hang](two_flat_pages_referencing_each_other_is_a_cycle_not_a_hang.md) — 7 ── Two flat pages that reference each other (neither reachable from root)
- [two_flat_siblings_merge_by_shared_power_label_root_stays_separate](two_flat_siblings_merge_by_shared_power_label_root_stays_separate.md) — 2 ── Two flat siblings (neither referenced by root nor by each other) merge
