---
okf_version: "0.2"
type: Class
title: LibraryBrowserState
description: Per-browser-tab state — owned by a single
resource: crates/oxide-app/src/library/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/mod/LibraryBrowserState
language: rust
---

# LibraryBrowserState

Per-browser-tab state — owned by a single

## Signature

```rust
pub struct LibraryBrowserState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Per-browser-tab state — owned by a single
`TabKind::LibraryBrowser(path)` tab, keyed by the same `path` on
`LibraryState::library_browsers`. Deliverable B adds the
`edit_modal` field so double-click on a row opens a full-form
editor.
[derive(Debug, Clone)]

## Methods

- `library_path`
- `active_table`
- `selected_row`
- `search`
- `lifecycle_filter`
- `class_filter`
- `edit_modal`
- `cell_edit`
- `delete_confirm`
- `sort_by`
- `adding_table`
- `delete_error`
- `renaming_table`
- `rename_error`
- `adding_class`
- `renaming_class`
- `class_error`

## Source
Lines 156–231 in `crates/oxide-app/src/library/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/state/mod.md) |
