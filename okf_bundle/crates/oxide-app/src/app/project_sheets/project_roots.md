---
okf_version: "0.2"
type: Function
title: project_roots
description: "The entry points [`oxide_net::build_project_netlist`] must walk: the"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/project_roots
language: rust
---

# project_roots

The entry points [`oxide_net::build_project_netlist`] must walk: the

## Signature

```rust
pub(crate) fn project_roots(
    root_key: oxide_net::SheetKey,
    project_set: &ProjectSheetSet,
    graph: &AssembledGraph,
    base_dir: Option<&Path>,
) -> Vec<oxide_net::ProjectRoot>
```

## Visibility

- `pub(crate)`

## Docstring

The entry points [`oxide_net::build_project_netlist`] must walk: the
project root first, then every declared page the root's hierarchy does not
reach (#430).

A flat project's second, third, … page has no `ChildSheet` reference
pointing at it — `Add Existing Sheet` produces exactly that — so it is
invisible to the stitcher unless it is named as a root in its own right.
Without this the netlist silently covers only the root's subtree: the
`.net` export refuses as incomplete, and the PDF still prints every page
while resolving `NET_NAME()` against a netlist those pages are absent from.

Every root walks with an empty name chain — a page is a *peer* of the root,
not nested under it, so its labels stay unqualified and a `VCC` on page two
is the same net as a `VCC` on page one. That is the whole point of stitching
the pages into one netlist, and it is why `ProjectRoot` carries no name
seed.

A page that another page *does* reach is still on this list, because
`pages_outside_the_hierarchy` means "not reachable **from the root**".
Listing it is *mostly* harmless: the stitcher's visited set walks it once
when the page referencing it sorts first, but twice in the reverse order
(#540, inherited from #430 and not introduced by the re-key).

Order is the caller's contract — it decides occurrence numbering and hence
`NetId` assignment, so it is part of the exported `.net` — and pages are
therefore sorted by their own [`oxide_net::SheetKey`], the identity every
other part of the graph is keyed by (#541).

Sorting `pages_outside_the_hierarchy` directly would be the obvious
shortcut and is wrong twice over. That `Vec` holds *absolute* paths
(`dir.join(&s.filename)`), and `Path`'s `Ord` is **component-wise** where
`SheetKey`'s is byte-wise over the whole string — a different comparison
function, not merely a different string. They disagree on an ordinary
nested project, no exotic input required: for `sub/a.snxsch` against
`sub-b.snxsch`, `Path` compares the component `sub` against `sub-b.snxsch`
and puts the nested page first, while the byte-wise compare reaches `-`
(`0x2d`) before `/` (`0x2f`) and puts the sibling first.

Keying the order to `SheetKey` also reproduces the pre-#466 order, which
sorted the children-map key `String`s — so an existing project's `.net` is
unchanged by the re-key rather than silently renumbered. And a relative,
base-anchored key cannot make net numbering depend on where the project
directory happens to sit on disk, which an absolute-path sort can for any
page declared outside it.

## Source
Lines 295–321 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| calls | [sheet_key](/crates/oxide-app/src/app/project_sheets/sheet_key.md) |
| called_by | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
| called_by | [pages_are_ordered_by_sheet_key_not_by_absolute_path](/crates/oxide-app/src/app/project_sheets/pages_are_ordered_by_sheet_key_not_by_absolute_path.md) |
