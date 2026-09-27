---
okf_version: "0.2"
type: Function
title: compare_references
description: "Natural designator order: `R1 < R2 < R9 < R10`, not `str::cmp`'s"
resource: crates/oxide-types/src/designator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/designator/compare_references
language: rust
---

# compare_references

Natural designator order: `R1 < R2 < R9 < R10`, not `str::cmp`'s

## Signature

```rust
pub fn compare_references(a: &str, b: &str) -> Ordering
```

## Visibility

- `pub`

## Docstring

Natural designator order: `R1 < R2 < R9 < R10`, not `str::cmp`'s
`R1 < R10 < R2`.

Both strings are walked as alternating digit / non-digit runs, so the rule
applies to every section of a multi-section designator (`U1_2 < U1_10`), not
just the first. Equal-under-the-rule inputs fall back to a byte compare so
the result is a total order (required by `sort_by`) and `R01` still differs
from `R1`.

## Source
Lines 59–88 in `crates/oxide-types/src/designator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [designator](/crates/oxide-types/src/designator.md) |
| calls | [take_run](/crates/oxide-types/src/designator/take_run.md) |
| calls | [compare_digit_runs](/crates/oxide-types/src/designator/compare_digit_runs.md) |
| calls | [compare_text_runs](/crates/oxide-types/src/designator/compare_text_runs.md) |
| called_by | [sorted_row_order](/crates/oxide-app/src/app/view/dialogs/bom/table/sorted_row_order.md) |
| called_by | [new](/crates/oxide-app/src/library/updates_dialog/new.md) |
| called_by | [build_table](/crates/oxide-bom/src/lib/build_table.md) |
| called_by | [build_project_netlist](/crates/oxide-net/src/project/mod/build_project_netlist.md) |
| called_by | [sorted](/crates/oxide-types/src/designator/sorted.md) |
