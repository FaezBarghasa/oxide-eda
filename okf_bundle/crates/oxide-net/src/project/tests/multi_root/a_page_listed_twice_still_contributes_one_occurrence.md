---
okf_version: "0.2"
type: Function
title: a_page_listed_twice_still_contributes_one_occurrence
description: "10 ── #466 × #430: a page listed twice contributes one occurrence, not two."
resource: crates/oxide-net/src/project/tests/multi_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/multi_root/a_page_listed_twice_still_contributes_one_occurrence
language: rust
---

# a_page_listed_twice_still_contributes_one_occurrence

10 ── #466 × #430: a page listed twice contributes one occurrence, not two.

## Signature

```rust
fn a_page_listed_twice_still_contributes_one_occurrence()
```

## Decorators

- `test`

## Docstring

10 ── #466 × #430: a page listed twice contributes one occurrence, not two.
The caller is not supposed to do this, but a page list assembled from
two sources could; double-stitching would corrupt the netlist quietly
rather than fail loudly, so the visited-set skip covers it too.
[test]

## Source
Lines 366–392 in `crates/oxide-net/src/project/tests/multi_root.rs`

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
| calls | [stitch_pages](/crates/oxide-net/src/project/tests/stitch_pages.md) |
