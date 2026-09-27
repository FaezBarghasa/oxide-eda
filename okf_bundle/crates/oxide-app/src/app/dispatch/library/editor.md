---
okf_version: "0.2"
type: Module
title: editor
description: "Primitive-editor handlers — opening `.snxsym` / `.snxfpt` document"
resource: crates/oxide-app/src/app/dispatch/library/editor.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor
language: rust
---

# editor

Primitive-editor handlers — opening `.snxsym` / `.snxfpt` document

## Docstring

Primitive-editor handlers — opening `.snxsym` / `.snxfpt` document
tabs, routing symbol / footprint edit events, saving primitive
tabs, and the associated canvas-cache / external-change plumbing.

Extracted verbatim from the library dispatcher (`dispatch/library`);
pure code motion, zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [handle_open_primitive_editor](/crates/oxide-app/src/app/dispatch/library/editor/handle_open_primitive_editor.md) |
| related | [handle_open_primitive](/crates/oxide-app/src/app/dispatch/library/editor/handle_open_primitive.md) |
| related | [open_primitive](/crates/oxide-app/src/app/dispatch/library/editor/open_primitive.md) |
| related | [handle_primitive_editor_event](/crates/oxide-app/src/app/dispatch/library/editor/handle_primitive_editor_event.md) |
| related | [handle_symbol_primitive_edit](/crates/oxide-app/src/app/dispatch/library/editor/handle_symbol_primitive_edit.md) |
| related | [handle_footprint_primitive_edit](/crates/oxide-app/src/app/dispatch/library/editor/handle_footprint_primitive_edit.md) |
| related | [invalidate_primitive_canvas_cache](/crates/oxide-app/src/app/dispatch/library/editor/invalidate_primitive_canvas_cache.md) |
| related | [save_primitive_tab_at](/crates/oxide-app/src/app/dispatch/library/editor/save_primitive_tab_at.md) |
| related | [commit_external_change_for](/crates/oxide-app/src/app/dispatch/library/editor/commit_external_change_for.md) |
| related | [refresh_primitive_cache_for](/crates/oxide-app/src/app/dispatch/library/editor/refresh_primitive_cache_for.md) |
| related | [reload_primitive_in_library_set](/crates/oxide-app/src/app/dispatch/library/editor/reload_primitive_in_library_set.md) |
| related | [handle_open_primitive_editor](/crates/oxide-app/src/app/dispatch/library/editor/handle_open_primitive_editor.md) |
| related | [handle_open_primitive](/crates/oxide-app/src/app/dispatch/library/editor/handle_open_primitive.md) |
| related | [open_primitive](/crates/oxide-app/src/app/dispatch/library/editor/open_primitive.md) |
| related | [handle_primitive_editor_event](/crates/oxide-app/src/app/dispatch/library/editor/handle_primitive_editor_event.md) |
| related | [handle_symbol_primitive_edit](/crates/oxide-app/src/app/dispatch/library/editor/handle_symbol_primitive_edit.md) |
| related | [handle_footprint_primitive_edit](/crates/oxide-app/src/app/dispatch/library/editor/handle_footprint_primitive_edit.md) |
| related | [invalidate_primitive_canvas_cache](/crates/oxide-app/src/app/dispatch/library/editor/invalidate_primitive_canvas_cache.md) |
| related | [save_primitive_tab_at](/crates/oxide-app/src/app/dispatch/library/editor/save_primitive_tab_at.md) |
| related | [commit_external_change_for](/crates/oxide-app/src/app/dispatch/library/editor/commit_external_change_for.md) |
| related | [refresh_primitive_cache_for](/crates/oxide-app/src/app/dispatch/library/editor/refresh_primitive_cache_for.md) |
| related | [reload_primitive_in_library_set](/crates/oxide-app/src/app/dispatch/library/editor/reload_primitive_in_library_set.md) |
