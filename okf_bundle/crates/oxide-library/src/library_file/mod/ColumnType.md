---
okf_version: "0.2"
type: Class
title: ColumnType
description: "Per-column type declared in `[tables.<name>.column_types]`."
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod/ColumnType
language: rust
---

# ColumnType

Per-column type declared in `[tables.<name>.column_types]`.

## Signature

```rust
pub enum ColumnType
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Per-column type declared in `[tables.<name>.column_types]`.

Drives:
* Numeric sort when a column is `Number` / `Int` (so `10k`-style
cells still sort as numbers when the column is typed numeric;
raw lexical sort gives `1, 10, 100, 2, 200, 25` which is the
Altium pain point we explicitly want to avoid).
* Validation in the Edit modal — `Bool` accepts only `true`/`false`,
`Enum` accepts only the declared values, etc.
* Render hints in the Library Browser — right-align numbers,
render `Bool` as checkboxes, render `Url` as clickable links,
render `Enum` as inline dropdowns.

Encoded in TOML as a string token (`"string"`, `"number"`, `"int"`,
`"bool"`, `"uuid"`, `"date"`, `"datetime"`, `"url"`, `"version"`,
`"tags"`, or `"enum:val1,val2,..."`). Keeping the wire format as
plain strings avoids inline-table noise in the `.snxlib`.
[derive(Debug, Clone, PartialEq, Eq)]

## Source
Lines 136–159 in `crates/oxide-library/src/library_file/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_file](/crates/oxide-library/src/library_file/mod.md) |
