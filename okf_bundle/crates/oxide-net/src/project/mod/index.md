# mod

## Classs

- [Analysis](Analysis.md) — Per-occurrence level-1 analysis, sampled after every level-1 union.
- [Occ](Occ.md) — One visited sheet in the hierarchy (an *occurrence* — the same file
- [ProjectGraph](ProjectGraph.md) — Pre-resolved input to [`build_project_netlist`]. The caller (the app) owns
- [ProjectNetlist](ProjectNetlist.md) — The whole-project netlist plus any structural issues found while stitching.
- [ProjectRoot](ProjectRoot.md) — One entry point the stitcher walks the hierarchy from.
- [RawNet](RawNet.md) — A net assembled from a level-2 group, before id assignment and dedup.
- [SheetKey](SheetKey.md) — Opaque, host-neutral identifier for one sheet within a [`ProjectGraph`] —
- [StitchIssue](StitchIssue.md) — A structural problem found while stitching. The netlist is still produced

## Functions

- [analyze](analyze.md) — Level-1 analysis for one sheet: the per-sheet derivation plus sheet-pin
- [as_str](as_str.md) — Borrow the underlying string — for display and for hashing into
- [as_str](as_str_1.md) — Borrow the underlying string — for display and for hashing into
- [assemble_net](assemble_net.md) — Assemble one final net from its level-2 member nodes. Returns `None` when
- [bucket_join](bucket_join.md) — Union `node` into the level-2 class of `name`, seeding the bucket the first
- [build_project_netlist](build_project_netlist.md) — Build the whole-project [`Netlist`] by stitching `graph.roots` down
- [detect_duplicate_uuids](detect_duplicate_uuids.md) — Report every pair of sheets that share a schematic uuid — copy-as-template
- [detect_shared_references](detect_shared_references.md) — Report each reference designator carried by a sheet key instantiated more
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [new](new.md) — Wrap a string the caller has already resolved and normalized.
- [new](new_1.md) — Wrap a string the caller has already resolved and normalized.
- [visit](visit.md)
