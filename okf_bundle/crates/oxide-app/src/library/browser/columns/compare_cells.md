---
okf_version: "0.2"
type: Function
title: compare_cells
description: Comparator for two cell strings with auto-detected numeric
resource: crates/oxide-app/src/library/browser/columns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/columns/compare_cells
language: rust
---

# compare_cells

Comparator for two cell strings with auto-detected numeric

## Signature

```rust
pub(super) fn compare_cells(a: &str, b: &str) -> std::cmp::Ordering
```

## Visibility

- `pub(super)`

## Docstring

Comparator for two cell strings with auto-detected numeric
fallback. If both values parse as `f64`, sort numerically;
otherwise sort case-insensitively. This is Stage 8's answer to
the Altium "lexical sort on numeric columns" pain — we don't
need a typed schema lookup at compare time, and untyped legacy
columns get the right behaviour automatically when their cells
happen to be numeric.

## Source
Lines 111–116 in `crates/oxide-app/src/library/browser/columns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [columns](/crates/oxide-app/src/library/browser/columns.md) |
| called_by | [view](/crates/oxide-app/src/library/browser/mod/view.md) |
