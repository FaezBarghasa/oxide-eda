---
okf_version: "0.2"
type: Function
title: handle_browser_delete_row_request
description: Phase 2 — open the delete-row confirm modal. Records
resource: crates/oxide-app/src/app/dispatch/library/browser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_delete_row_request_1
language: rust
---

# handle_browser_delete_row_request

Phase 2 — open the delete-row confirm modal. Records

## Signature

```rust
pub(super) fn handle_browser_delete_row_request(
        &mut self,
        library_path: std::path::PathBuf,
        table: String,
        row_id: RowId,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Phase 2 — open the delete-row confirm modal. Records
`(table, row_id, internal_pn)` on the browser state so the
modal can render a confident message.

## Source
Lines 279–300 in `crates/oxide-app/src/app/dispatch/library/browser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [browser](/crates/oxide-app/src/app/dispatch/library/browser/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
