---
okf_version: "0.2"
type: Function
title: a_page_referenced_by_a_later_sorting_page_is_still_stitched_once
description: "11 ── #540. Test 5 proved the visited-set skip works when the *referencing*"
resource: crates/oxide-net/src/project/tests/multi_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/multi_root/a_page_referenced_by_a_later_sorting_page_is_still_stitched_once
language: rust
---

# a_page_referenced_by_a_later_sorting_page_is_still_stitched_once

11 ── #540. Test 5 proved the visited-set skip works when the *referencing*

## Signature

```rust
fn a_page_referenced_by_a_later_sorting_page_is_still_stitched_once()
```

## Decorators

- `test`

## Docstring

11 ── #540. Test 5 proved the visited-set skip works when the *referencing*
page is walked first. Sorted page order decides that, and page names
have nothing to do with who references whom — so here the referenced
page sorts first, and the skip has to be arranged rather than
inherited. Walking it twice would duplicate `R_A`, raise a spurious
`SharedReferenceAcrossInstances`, and shift every subsequent `NetId`.
[test]

## Source
Lines 434–459 in `crates/oxide-net/src/project/tests/multi_root.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_root](/crates/oxide-net/src/project/tests/multi_root.md) |
| calls | [stitch_referencing_page_last](/crates/oxide-net/src/project/tests/multi_root/stitch_referencing_page_last.md) |
