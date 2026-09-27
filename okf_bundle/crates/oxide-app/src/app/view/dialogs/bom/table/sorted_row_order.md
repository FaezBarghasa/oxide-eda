---
okf_version: "0.2"
type: Function
title: sorted_row_order
description: "Row indexes into `rows` in display order."
resource: crates/oxide-app/src/app/view/dialogs/bom/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/bom/table/sorted_row_order
language: rust
---

# sorted_row_order

Row indexes into `rows` in display order.

## Signature

```rust
fn sorted_row_order(
    rows: &[oxide_output::BomRow],
    columns: &[oxide_output::BomColumn],
    sort: Option<(usize, bool)>,
) -> Vec<usize>
```

## Docstring

Row indexes into `rows` in display order.

Indexes rather than rows so the caller borrows without cloning the `BomRow`
vec. Qty sorts numerically; a designator column sorts with the *same*
natural order the export pipeline applies (`oxide_bom::build_table`), so
the modal and the CSV/HTML/XLSX the user exports next never disagree — a
plain `str` compare here would show R1, R10, R2 over an export reading
R1, R2, R10. Every other column sorts case-insensitively on its cell text.

## Source
Lines 40–70 in `crates/oxide-app/src/app/view/dialogs/bom/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/app/view/dialogs/bom/table.md) |
| calls | [compare_references](/crates/oxide-types/src/designator/compare_references.md) |
| calls | [first_reference](/crates/oxide-app/src/app/view/dialogs/bom/table/first_reference.md) |
| calls | [column_value](/crates/oxide-app/src/app/view/dialogs/bom/table/column_value.md) |
| called_by | [bom_table](/crates/oxide-app/src/app/view/dialogs/bom/table/bom_table.md) |
| called_by | [designator_sort_matches_the_export_order](/crates/oxide-app/src/app/view/dialogs/bom/table/designator_sort_matches_the_export_order.md) |
