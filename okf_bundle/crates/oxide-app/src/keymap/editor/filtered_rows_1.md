---
okf_version: "0.2"
type: Function
title: filtered_rows
description: "[`Self::rows`] filtered by a case-insensitive search query against"
resource: crates/oxide-app/src/keymap/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/keymap/editor/filtered_rows_1
language: rust
---

# filtered_rows

[`Self::rows`] filtered by a case-insensitive search query against

## Signature

```rust
pub fn filtered_rows(&self, query: &str) -> Vec<KeymapEditorRow>
```

## Visibility

- `pub`

## Docstring

[`Self::rows`] filtered by a case-insensitive search query against
each row's label, command id or trigger text. An empty query
returns every row.

## Source
Lines 95–100 in `crates/oxide-app/src/keymap/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/keymap/editor.md) |
