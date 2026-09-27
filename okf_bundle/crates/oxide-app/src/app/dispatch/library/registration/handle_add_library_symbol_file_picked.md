---
okf_version: "0.2"
type: Function
title: handle_add_library_symbol_file_picked
description: F34 — Save-As dialog confirmed for a new symbol library file
resource: crates/oxide-app/src/app/dispatch/library/registration.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/registration/handle_add_library_symbol_file_picked
language: rust
---

# handle_add_library_symbol_file_picked

F34 — Save-As dialog confirmed for a new symbol library file

## Signature

```rust
impl Oxide { pub(crate) fn handle_add_library_symbol_file_picked(
        &mut self,
        path: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

F34 — Save-As dialog confirmed for a new symbol library file
(`.snxsym`). The user picked the location + filename in the
rfd `save_file()` dialog — that click IS the explicit save
action, so we write the empty `SymbolFile` to disk
immediately, register the path on the containing project's
`data.libraries` list (so the tree shows it directly under
Libraries), then open it as a clean primitive editor tab
(dirty=false). Subsequent edits flow through the regular
`Ctrl+S → save_primitive_tab_at` path.

## Source
Lines 270–316 in `crates/oxide-app/src/app/dispatch/library/registration.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [registration](/crates/oxide-app/src/app/dispatch/library/registration.md) |
| calls | [atomic_write](/crates/oxide-app/src/app/dispatch/library/recovery/atomic_write.md) |
