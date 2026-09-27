---
okf_version: "0.2"
type: Function
title: build_expression_tables
description: Builds the PDF/preview expression tables. Net names come from the
resource: crates/oxide-output/src/expression.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T19:55:16Z"
concept_id: crates/oxide-output/src/expression/build_expression_tables
language: rust
---

# build_expression_tables

Builds the PDF/preview expression tables. Net names come from the

## Signature

```rust
pub fn build_expression_tables(
    sheets: &[SheetSnapshot],
    netlist: Option<&Netlist>,
) -> ExpressionTables
```

## Visibility

- `pub`

## Docstring

Builds the PDF/preview expression tables. Net names come from the
project's authoritative `Netlist` (ADR-0002 D7) rather than a
PDF-local re-derivation — `netlist` is `None` when the caller didn't
derive one (e.g. a PDF-only export with no netlist attached), in which
case `net_name_by_symbol_pin` is empty rather than guessing.

## Source
Lines 19–27 in `crates/oxide-output/src/expression.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [expression](/crates/oxide-output/src/expression.md) |
| calls | [build_global_refdes_lookup](/crates/oxide-output/src/expression/build_global_refdes_lookup.md) |
| calls | [build_pin_net_lookup](/crates/oxide-output/src/expression/build_pin_net_lookup.md) |
| called_by | [export](/crates/oxide-output/src/pdf/mod/export.md) |
| called_by | [rasterize](/crates/oxide-output/src/preview/mod/rasterize.md) |
