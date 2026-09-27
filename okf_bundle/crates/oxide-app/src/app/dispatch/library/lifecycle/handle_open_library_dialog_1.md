---
okf_version: "0.2"
type: Function
title: handle_open_library_dialog
description: "File ▸ Library ▸ Open Library… — runs `rfd::AsyncFileDialog` on"
resource: crates/oxide-app/src/app/dispatch/library/lifecycle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/lifecycle/handle_open_library_dialog_1
language: rust
---

# handle_open_library_dialog

File ▸ Library ▸ Open Library… — runs `rfd::AsyncFileDialog` on

## Signature

```rust
pub(super) fn handle_open_library_dialog(&mut self) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

File ▸ Library ▸ Open Library… — runs `rfd::AsyncFileDialog` on
the directory level and lands in [`LibraryMessage::OpenLibraryAt`].

## Source
Lines 13–24 in `crates/oxide-app/src/app/dispatch/library/lifecycle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lifecycle](/crates/oxide-app/src/app/dispatch/library/lifecycle.md) |
