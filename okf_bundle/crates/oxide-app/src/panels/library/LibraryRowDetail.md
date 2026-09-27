---
okf_version: "0.2"
type: Class
title: LibraryRowDetail
description: Detail for the row currently selected in the active Library
resource: crates/oxide-app/src/panels/library.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/library/LibraryRowDetail
language: rust
---

# LibraryRowDetail

Detail for the row currently selected in the active Library

## Signature

```rust
pub struct LibraryRowDetail
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Detail for the row currently selected in the active Library
Browser tab. Surfaces in the right-edge Properties panel so
primitive binding (Pick Symbol / Pick Footprint) lives next to
every other "what am I selected?" affordance the user looks for
there. F15 (2026-05-03 library polish): "right pane can be opened
on properties instead."

Populated by `refresh_panel_ctx` when the active tab is
`TabKind::LibraryBrowser(path)` AND the matching browser state's
`selected_row` is `Some(_)`. Cleared otherwise.
[derive(Debug, Clone)]

## Methods

- `library_path`
- `table`
- `row_id`
- `internal_pn`
- `class`
- `lifecycle_label`
- `symbol_summary`
- `footprint_summary`

## Source
Lines 26–43 in `crates/oxide-app/src/panels/library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library](/crates/oxide-app/src/panels/library.md) |
