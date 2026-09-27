---
okf_version: "0.2"
type: Function
title: handle_open_primitive_editor
description: "Open a standalone primitive editor tab for the file at `path`."
resource: crates/oxide-app/src/app/dispatch/library/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/handle_open_primitive_editor
language: rust
---

# handle_open_primitive_editor

Open a standalone primitive editor tab for the file at `path`.

## Signature

```rust
impl Oxide { pub(super) fn handle_open_primitive_editor(
        &mut self,
        path: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Open a standalone primitive editor tab for the file at `path`.
Fired by the Component Preview tab's right-click context menu on
the Symbol / Footprint render panes. Standalone tab opens in WS-7.

## Source
Lines 17–27 in `crates/oxide-app/src/app/dispatch/library/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/app/dispatch/library/editor.md) |
