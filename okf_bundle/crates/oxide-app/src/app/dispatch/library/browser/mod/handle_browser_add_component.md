---
okf_version: "0.2"
type: Function
title: handle_browser_add_component
description: "Inline \"+ Component\" — mint a draft row directly into the"
resource: crates/oxide-app/src/app/dispatch/library/browser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_add_component
language: rust
---

# handle_browser_add_component

Inline "+ Component" — mint a draft row directly into the

## Signature

```rust
impl Oxide { pub(super) fn handle_browser_add_component(
        &mut self,
        library_path: std::path::PathBuf,
        table: Option<String>,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Inline "+ Component" — mint a draft row directly into the
browser's active table without opening any modal. The user
fills in `internal_pn`, `manufacturer`, `mpn` etc. via the
grid's inline cell editor; symbol / footprint binding lives in
the Properties panel for the selected row.

Library is implicit (the browser tab's library), table is the
browser's `active_table` or — when none is selected — the
generic-class default resolved through
`manifest.table_for_class("generic")`. Closes F17 / F18 of the
2026-05-03 library polish: the New Component modal's library
dropdown was meaningless inside a library tab, and the modal
itself was a step the user didn't want.

## Source
Lines 169–274 in `crates/oxide-app/src/app/dispatch/library/browser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [browser](/crates/oxide-app/src/app/dispatch/library/browser/mod.md) |
| calls | [create_component_row](/crates/oxide-app/src/library/commands/create_component_row.md) |
