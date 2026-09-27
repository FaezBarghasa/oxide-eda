---
okf_version: "0.2"
type: Function
title: stitch_referencing_page_last
description: "Test 5's topology with the names swapped: the referenced page is"
resource: crates/oxide-net/src/project/tests/multi_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/multi_root/stitch_referencing_page_last
language: rust
---

# stitch_referencing_page_last

Test 5's topology with the names swapped: the referenced page is

## Signature

```rust
fn stitch_referencing_page_last(pages: &[&str]) -> super::super::ProjectNetlist
```

## Docstring

Test 5's topology with the names swapped: the referenced page is
`a.snxsch` and the page referencing it is `z.snxsch`, so sorted page order
hands the *referenced* one over first. Takes the page order so the two
tests below can vary only that.

## Source
Lines 398–425 in `crates/oxide-net/src/project/tests/multi_root.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_root](/crates/oxide-net/src/project/tests/multi_root.md) |
| calls | [empty_sheet](/crates/oxide-net/src/project/tests/empty_sheet.md) |
| calls | [wire](/crates/oxide-net/src/project/tests/wire.md) |
| calls | [pt](/crates/oxide-net/src/project/tests/pt.md) |
| calls | [label](/crates/oxide-net/src/project/tests/label.md) |
| calls | [add_lib](/crates/oxide-net/src/project/tests/add_lib.md) |
| calls | [place](/crates/oxide-net/src/project/tests/place.md) |
| calls | [child_sheet](/crates/oxide-net/src/project/tests/child_sheet.md) |
| calls | [stitch_pages](/crates/oxide-net/src/project/tests/stitch_pages.md) |
| called_by | [a_page_referenced_by_a_later_sorting_page_is_still_stitched_once](/crates/oxide-net/src/project/tests/multi_root/a_page_referenced_by_a_later_sorting_page_is_still_stitched_once.md) |
