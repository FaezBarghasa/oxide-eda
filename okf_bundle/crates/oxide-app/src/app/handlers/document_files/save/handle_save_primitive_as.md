---
okf_version: "0.2"
type: Function
title: handle_save_primitive_as
description: "Resolve `Message::File(FileMsg::SavePrimitiveAs { from_path, to_path })` —"
resource: crates/oxide-app/src/app/handlers/document_files/save.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/save/handle_save_primitive_as
language: rust
---

# handle_save_primitive_as

Resolve `Message::File(FileMsg::SavePrimitiveAs { from_path, to_path })` —

## Signature

```rust
impl Oxide { pub(crate) fn handle_save_primitive_as(
        &mut self,
        from_path: &std::path::Path,
        to_path: &std::path::Path,
    ) }
```

## Visibility

- `pub(crate)`

## Docstring

Resolve `Message::File(FileMsg::SavePrimitiveAs { from_path, to_path })` —
re-key the editor and tab from the in-memory `from_path` to
the user-chosen `to_path`, then write the file via
`save_primitive_tab_at`. Same machinery the in-memory editor
already uses for atomic writes; just runs it under the new
path the user picked.

## Source
Lines 279–371 in `crates/oxide-app/src/app/handlers/document_files/save.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [save](/crates/oxide-app/src/app/handlers/document_files/save.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
