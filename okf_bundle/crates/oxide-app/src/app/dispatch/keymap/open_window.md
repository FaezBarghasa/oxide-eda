---
okf_version: "0.2"
type: Function
title: open_window
resource: crates/oxide-app/src/app/dispatch/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/keymap/open_window
language: rust
---

# open_window

## Signature

```rust
fn open_window(app: &mut Oxide, kind: WindowKind) -> iced::window::Id
```

## Source
Lines 182–186 in `crates/oxide-app/src/app/dispatch/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/app/dispatch/keymap.md) |
| called_by | [undocked](/crates/oxide-app/src/app/dispatch/keymap/undocked.md) |
| called_by | [windows_that_paint_no_document_get_global_only](/crates/oxide-app/src/app/dispatch/keymap/windows_that_paint_no_document_get_global_only.md) |
