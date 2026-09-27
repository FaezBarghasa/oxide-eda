---
okf_version: "0.2"
type: Function
title: sort_key
description: "Stable sort key matching `LibraryMessage::BrowserSortColumn`'s"
resource: crates/oxide-app/src/library/browser/columns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/columns/sort_key
language: rust
---

# sort_key

Stable sort key matching `LibraryMessage::BrowserSortColumn`'s

## Signature

```rust
impl ColumnKind { pub(super) fn sort_key(&self) -> String }
```

## Visibility

- `pub(super)`

## Docstring

Stable sort key matching `LibraryMessage::BrowserSortColumn`'s
`column_key` field. Ties columns to their cell-edit buffers
and to the [`super::super::state::BrowserSort`] state.

## Source
Lines 52–63 in `crates/oxide-app/src/library/browser/columns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [columns](/crates/oxide-app/src/library/browser/columns.md) |
