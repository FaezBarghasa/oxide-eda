---
okf_version: "0.2"
type: Function
title: handle_open_primitive
description: "Open a `.snxsym` or `.snxfpt` as a main-window document tab."
resource: crates/oxide-app/src/app/dispatch/library/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/handle_open_primitive_1
language: rust
---

# handle_open_primitive

Open a `.snxsym` or `.snxfpt` as a main-window document tab.

## Signature

```rust
pub(crate) fn handle_open_primitive(&mut self, path: std::path::PathBuf) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Open a `.snxsym` or `.snxfpt` as a main-window document tab.
Reads the file from disk, builds the matching editor state,
and pushes a `TabKind::SymbolEditor(path)` /
`FootprintEditor(path)` tab into `DocumentState.tabs`.

Activates an existing tab when the same path is already open
instead of duplicating.

The error boundary for [`Self::open_primitive`] (#532). Every
failure leg used to be a `tracing::warn!` and a bare return: the
user double-clicked a corrupt `.snxsym` in the project tree and
**nothing happened** — no tab, no card, indistinguishable from a
mis-click. The warning did reach the Messages panel, but that is
a panel you have to already suspect something to open.

Routes the failure the way `handle_document_file_opened` already
does, plus the shared error card so it is impossible to miss.

## Source
Lines 46–56 in `crates/oxide-app/src/app/dispatch/library/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/app/dispatch/library/editor.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
