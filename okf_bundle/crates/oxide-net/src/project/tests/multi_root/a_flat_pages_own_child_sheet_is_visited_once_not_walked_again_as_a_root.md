---
okf_version: "0.2"
type: Function
title: a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root
description: 5 ── THE VISITED-SET CASE. A flat page can itself have its own child sheet
resource: crates/oxide-net/src/project/tests/multi_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/multi_root/a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root
language: rust
---

# a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root

5 ── THE VISITED-SET CASE. A flat page can itself have its own child sheet

## Signature

```rust
fn a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root()
```

## Decorators

- `test`

## Docstring

5 ── THE VISITED-SET CASE. A flat page can itself have its own child sheet
(a hierarchy hanging off a flat page). Because `pages_outside_the_hierarchy`
is "declared and not reachable **from the root**", that child is on the
caller's page list too — the root reaches neither. It must still be
visited exactly once, through its parent page's subtree, and NOT walked
a second time as a root in its own right.

A second visit would not merely be redundant: it duplicates the sheet's
terminals, raises a spurious `SharedReferenceAcrossInstances`, and
shifts every subsequent `NetId` (ids are positional).
[test]

## Source
Lines 225–276 in `crates/oxide-net/src/project/tests/multi_root.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_root](/crates/oxide-net/src/project/tests/multi_root.md) |
| calls | [empty_sheet](/crates/oxide-net/src/project/tests/empty_sheet.md) |
| calls | [wire](/crates/oxide-net/src/project/tests/wire.md) |
| calls | [pt](/crates/oxide-net/src/project/tests/pt.md) |
| calls | [add_lib](/crates/oxide-net/src/project/tests/add_lib.md) |
| calls | [place](/crates/oxide-net/src/project/tests/place.md) |
| calls | [child_sheet](/crates/oxide-net/src/project/tests/child_sheet.md) |
| calls | [label](/crates/oxide-net/src/project/tests/label.md) |
| calls | [stitch_pages](/crates/oxide-net/src/project/tests/stitch_pages.md) |
| calls | [find](/crates/oxide-net/src/uf/find.md) |
