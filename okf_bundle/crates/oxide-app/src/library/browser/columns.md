---
okf_version: "0.2"
type: Module
title: columns
description: Library Browser — grid column model + derivation.
resource: crates/oxide-app/src/library/browser/columns.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/columns
language: rust
---

# columns

Library Browser — grid column model + derivation.

## Docstring

Library Browser — grid column model + derivation.

The `GridColumn` / `ColumnKind` types, the numeric-aware cell
comparator, `derive_columns` (always-on + template-derived +
most-common parametric columns), the lifecycle dot colour and the
search-filter predicate. Extracted verbatim from the former
single-file `browser` module.

## Relationships

| Type | Target |
|------|--------|
| related | [GridColumn](/crates/oxide-app/src/library/browser/columns/GridColumn.md) |
| related | [ColumnKind](/crates/oxide-app/src/library/browser/columns/ColumnKind.md) |
| related | [sort_key](/crates/oxide-app/src/library/browser/columns/sort_key.md) |
| related | [cell_value](/crates/oxide-app/src/library/browser/columns/cell_value.md) |
| related | [sort_key](/crates/oxide-app/src/library/browser/columns/sort_key.md) |
| related | [cell_value](/crates/oxide-app/src/library/browser/columns/cell_value.md) |
| related | [compare_cells](/crates/oxide-app/src/library/browser/columns/compare_cells.md) |
| related | [derive_columns](/crates/oxide-app/src/library/browser/columns/derive_columns.md) |
| related | [lifecycle_dot_color](/crates/oxide-app/src/library/browser/columns/lifecycle_dot_color.md) |
| related | [shorten_label](/crates/oxide-app/src/library/browser/columns/shorten_label.md) |
| related | [row_matches_filter](/crates/oxide-app/src/library/browser/columns/row_matches_filter.md) |
