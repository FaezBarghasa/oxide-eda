---
okf_version: "0.2"
type: Function
title: handle_bom_preview_sort_column
description: Click on a header cell — cycle through ascending → descending
resource: crates/oxide-app/src/app/handlers/menu/export/bom.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/bom/handle_bom_preview_sort_column_1
language: rust
---

# handle_bom_preview_sort_column

Click on a header cell — cycle through ascending → descending

## Signature

```rust
pub(crate) fn handle_bom_preview_sort_column(&mut self, idx: usize)
```

## Visibility

- `pub(crate)`

## Docstring

Click on a header cell — cycle through ascending → descending
→ no-sort. The previewed table rows get sorted live in
`view_bom_preview`; we don't re-export-order the underlying
table because sorting is a render-only convenience and the
final exported file should mirror the user's chosen sort, not
the rollup default.

## Source
Lines 129–142 in `crates/oxide-app/src/app/handlers/menu/export/bom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-app/src/app/handlers/menu/export/bom.md) |
