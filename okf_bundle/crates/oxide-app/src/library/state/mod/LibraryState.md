---
okf_version: "0.2"
type: Class
title: LibraryState
description: Top-level Library subsystem state. Stored on
resource: crates/oxide-app/src/library/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/mod/LibraryState
language: rust
---

# LibraryState

Top-level Library subsystem state. Stored on

## Signature

```rust
pub struct LibraryState
```

## Visibility

- `pub`

## Docstring

Top-level Library subsystem state. Stored on
[`crate::app::Oxide`] as a single field so the dispatcher can
borrow it independently of the rest of `DocumentState`.

## Methods

- `set`
- `open_libraries`
- `editors`
- `where_used`
- `picker`
- `settings`
- `expanded`
- `panel_search`
- `new_component`
- `close_library_confirm`
- `template_registry`
- `library_browsers`
- `primitive_picker`
- `document_options`
- `recovery`
- `create_options`
- `library_updates`
- `skipped_updates_for`
- `installed_libraries`
- `global_libraries`
- `components_panel`
- `pending_mounts`

## Source
Lines 391–498 in `crates/oxide-app/src/library/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/state/mod.md) |
