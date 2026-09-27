---
okf_version: "0.2"
type: Function
title: designator_sort_matches_the_export_order
description: The export pipeline orders rollup rows by their first designator with
resource: crates/oxide-app/src/app/view/dialogs/bom/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/bom/table/designator_sort_matches_the_export_order
language: rust
---

# designator_sort_matches_the_export_order

The export pipeline orders rollup rows by their first designator with

## Signature

```rust
fn designator_sort_matches_the_export_order()
```

## Decorators

- `test`

## Docstring

The export pipeline orders rollup rows by their first designator with
`oxide_types::designator::compare_references`, and the preview modal
opens with the Designator column sort pre-seeded. Sorting the joined
reference string with `str::cmp` here showed R1, R10, R2 in the modal
over a CSV/HTML/XLSX reading R1, R2, R10 — the preview has to agree
with the file the user exports one click later.
[test]

## Source
Lines 395–408 in `crates/oxide-app/src/app/view/dialogs/bom/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/app/view/dialogs/bom/table.md) |
| calls | [sorted_row_order](/crates/oxide-app/src/app/view/dialogs/bom/table/sorted_row_order.md) |
