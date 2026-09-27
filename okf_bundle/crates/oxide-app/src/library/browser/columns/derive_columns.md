---
okf_version: "0.2"
type: Function
title: derive_columns
description: "Resolve the column list. Always: Internal PN / Manufacturer / MPN /"
resource: crates/oxide-app/src/library/browser/columns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/columns/derive_columns
language: rust
---

# derive_columns

Resolve the column list. Always: Internal PN / Manufacturer / MPN /

## Signature

```rust
pub(super) fn derive_columns(
    rows: &[ComponentRow],
    library_id: uuid::Uuid,
    registry: &oxide_library::TemplateRegistry,
    table_name: &str,
) -> Vec<GridColumn>
```

## Visibility

- `pub(super)`

## Docstring

Resolve the column list. Always: Internal PN / Manufacturer / MPN /
Rev / Symbol / Footprint. Then template-derived columns from the
`TemplateRegistry`: every `required_param` slot from the templates
resolved for the table's classes (de-duplicated across classes).
Then a Tags column (Stage 18) when *any* row carries a non-empty
`parameters["tags"]`. Finally up to [`MAX_PARAM_COLUMNS`] of the
most-common other parametric keys across `rows` — `tags` and any
already-shown template params are excluded so columns don't render
twice.

Template resolution sources `class` from the rows when present;
for an empty table it strips a trailing "s" off `table_name` and
uses that as the implicit class (works for the default
pluralisation `resistor` → `resistors` etc.). F19 / F20 of the
2026-05-03 library polish: the user wanted basic params per table
to appear by default, AND they want Tables to be the only
user-facing concept (Classes are now derived purely from the
table name, never edited directly).

## Source
Lines 136–266 in `crates/oxide-app/src/library/browser/columns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [columns](/crates/oxide-app/src/library/browser/columns.md) |
| calls | [shorten_label](/crates/oxide-app/src/library/browser/columns/shorten_label.md) |
| called_by | [view](/crates/oxide-app/src/library/browser/mod/view.md) |
