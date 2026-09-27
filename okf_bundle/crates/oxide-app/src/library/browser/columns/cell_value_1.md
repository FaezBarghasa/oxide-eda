---
okf_version: "0.2"
type: Function
title: cell_value
description: "Extract the row's cell value for this column. Empty string when"
resource: crates/oxide-app/src/library/browser/columns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/columns/cell_value_1
language: rust
---

# cell_value

Extract the row's cell value for this column. Empty string when

## Signature

```rust
pub(super) fn cell_value(&self, r: &ComponentRow) -> String
```

## Visibility

- `pub(super)`

## Docstring

Extract the row's cell value for this column. Empty string when
the row has no value for the underlying field. For
[`ColumnKind::Rev`], the value is the bare semver string (the
🔒/🔓 badge is added at render time, not in the sort key, so
released and unreleased rows still sort by version order).

## Source
Lines 70–101 in `crates/oxide-app/src/library/browser/columns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [columns](/crates/oxide-app/src/library/browser/columns.md) |
