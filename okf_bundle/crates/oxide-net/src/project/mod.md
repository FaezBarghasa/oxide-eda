---
okf_version: "0.2"
type: Module
title: project
description: "Cross-sheet netlist stitching — [`build_project_netlist`] (ADR-0002 D8,"
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod
language: rust
---

# project

Cross-sheet netlist stitching — [`build_project_netlist`] (ADR-0002 D8,

## Docstring

Cross-sheet netlist stitching — [`build_project_netlist`] (ADR-0002 D8,
A3.1 increment 2c; re-keyed by resolved path in #466).

A schematic project is a [`ProjectGraph`]: every sheet keyed by an opaque,
caller-resolved [`SheetKey`], plus a per-parent resolution map from each
`ChildSheet.filename` string to the child's key. The app resolves every
reference against its own parent's directory before handing this crate the
graph — this crate never joins paths, normalizes separators, or folds case;
it only compares keys. Which host decisions the caller applies (and where
it stops) is the caller's business, documented at its own key-assembly
step; nothing here depends on them. This module walks the hierarchy from
`graph.roots` and derives one [`Netlist`] for the whole design, on top of
the same per-sheet analysis [`build_netlist`](crate::build_netlist) uses —
so one root with an empty `resolved` map is byte-identical to
`build_netlist(root)`.

**Multi-root / flat-stitch traversal (#430):** `graph.roots` is walked as
ordered by `root_order::order_roots`, each root an independent top-level
page — own subtree, own cycle-detection path, own name-chain seed; a peer
of the first root, not nested under it. A root already reached as an
earlier root's child is skipped rather than walked twice, so a declared
page that another page also references contributes exactly one occurrence
whichever way the two are named (#540). This is what lets a flat,
multi-page project — several sibling sheets, none referencing any other,
which is what `Add Existing Sheet` produces — stitch into one netlist
instead of leaving every page but the first out of it.

Two-level union-find: **level 1** is the per-sheet derivation (wires,
junctions, on-sheet label merge) plus sheet-pin anchoring; **level 2** joins
the resulting per-occurrence net roots across the project by three rules —
same-name Global/Power labels, power-port symbols as global name carriers,
and sheet-pin ↔ child-label binding. These rules run over *every*
occurrence regardless of which root's subtree it came from, so cross-page
merging for a flat project falls out of the existing machinery unchanged —
no new net model. Structural problems are reported as [`StitchIssue`]s
in-band; the netlist is always produced, deterministically.

## Relationships

| Type | Target |
|------|--------|
| related | [SheetKey](/crates/oxide-net/src/project/mod/SheetKey.md) |
| related | [new](/crates/oxide-net/src/project/mod/new.md) |
| related | [as_str](/crates/oxide-net/src/project/mod/as_str.md) |
| related | [new](/crates/oxide-net/src/project/mod/new.md) |
| related | [as_str](/crates/oxide-net/src/project/mod/as_str.md) |
| related | [fmt](/crates/oxide-net/src/project/mod/fmt.md) |
| related | [fmt](/crates/oxide-net/src/project/mod/fmt.md) |
| related | [ProjectRoot](/crates/oxide-net/src/project/mod/ProjectRoot.md) |
| related | [ProjectGraph](/crates/oxide-net/src/project/mod/ProjectGraph.md) |
| related | [StitchIssue](/crates/oxide-net/src/project/mod/StitchIssue.md) |
| related | [ProjectNetlist](/crates/oxide-net/src/project/mod/ProjectNetlist.md) |
| related | [Occ](/crates/oxide-net/src/project/mod/Occ.md) |
| related | [Analysis](/crates/oxide-net/src/project/mod/Analysis.md) |
| related | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
| related | [RawNet](/crates/oxide-net/src/project/mod/RawNet.md) |
| related | [assemble_net](/crates/oxide-net/src/project/mod/assemble_net.md) |
| related | [analyze](/crates/oxide-net/src/project/mod/analyze.md) |
| related | [visit](/crates/oxide-net/src/project/mod/visit.md) |
| related | [bucket_join](/crates/oxide-net/src/project/mod/bucket_join.md) |
| related | [detect_shared_references](/crates/oxide-net/src/project/mod/detect_shared_references.md) |
| related | [detect_duplicate_uuids](/crates/oxide-net/src/project/mod/detect_duplicate_uuids.md) |
