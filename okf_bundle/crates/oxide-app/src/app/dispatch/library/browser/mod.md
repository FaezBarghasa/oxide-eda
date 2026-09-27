---
okf_version: "0.2"
type: Module
title: browser
description: "Library Browser tab handlers — opening the browser, adding /"
resource: crates/oxide-app/src/app/dispatch/library/browser/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/mod
language: rust
---

# browser

Library Browser tab handlers — opening the browser, adding /

## Docstring

Library Browser tab handlers — opening the browser, adding /
deleting / editing component rows, inline cell commits, and
opening a Component Preview row from the browser.

Extracted verbatim from the library dispatcher (`dispatch/library`);
pure code motion, zero behaviour change.

The folder carries the `browser` namespace: class handlers live in
`classes`, grid/row handlers in `grid`, and table handlers in
`tables`; this `mod.rs` keeps the browser-tab handlers.

## Relationships

| Type | Target |
|------|--------|
| related | [handle_open_library_browser](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_open_library_browser.md) |
| related | [finish_open_library_browser](/crates/oxide-app/src/app/dispatch/library/browser/mod/finish_open_library_browser.md) |
| related | [handle_browser_add_component](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_add_component.md) |
| related | [handle_browser_delete_row_request](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_delete_row_request.md) |
| related | [handle_browser_delete_row_confirm](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_delete_row_confirm.md) |
| related | [handle_browser_open_edit_modal](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_open_edit_modal.md) |
| related | [handle_browser_edit_msg](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_edit_msg.md) |
| related | [handle_browser_cell_commit](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_cell_commit.md) |
| related | [handle_open_component_row](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_open_component_row.md) |
| related | [handle_open_library_browser](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_open_library_browser.md) |
| related | [finish_open_library_browser](/crates/oxide-app/src/app/dispatch/library/browser/mod/finish_open_library_browser.md) |
| related | [handle_browser_add_component](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_add_component.md) |
| related | [handle_browser_delete_row_request](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_delete_row_request.md) |
| related | [handle_browser_delete_row_confirm](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_delete_row_confirm.md) |
| related | [handle_browser_open_edit_modal](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_open_edit_modal.md) |
| related | [handle_browser_edit_msg](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_edit_msg.md) |
| related | [handle_browser_cell_commit](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_cell_commit.md) |
| related | [handle_open_component_row](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_open_component_row.md) |
