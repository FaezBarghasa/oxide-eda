---
okf_version: "0.2"
type: Function
title: handle_primitive_editor_event
description: "Apply a primitive-editor inner message to the matching tab's"
resource: crates/oxide-app/src/app/dispatch/library/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/handle_primitive_editor_event
language: rust
---

# handle_primitive_editor_event

Apply a primitive-editor inner message to the matching tab's

## Signature

```rust
impl Oxide { pub(crate) fn handle_primitive_editor_event(
        &mut self,
        path: std::path::PathBuf,
        msg: PrimitiveEdit,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Apply a primitive-editor inner message to the matching tab's
editor state. Path-keyed lookup distinguishes Symbol vs
Footprint; the dispatcher routes to the existing canvas-state
helpers so the standalone tab behaviour matches the in-Component
Editor experience verbatim.

## Source
Lines 206–233 in `crates/oxide-app/src/app/dispatch/library/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/app/dispatch/library/editor.md) |
| calls | [spawn_save_as_for_new_primitive](/crates/oxide-app/src/app/handlers/document_files/mod/spawn_save_as_for_new_primitive.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
