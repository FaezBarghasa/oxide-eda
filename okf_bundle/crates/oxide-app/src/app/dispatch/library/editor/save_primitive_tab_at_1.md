---
okf_version: "0.2"
type: Function
title: save_primitive_tab_at
description: "Write the primitive at `path` back to disk as JSON, commit"
resource: crates/oxide-app/src/app/dispatch/library/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/save_primitive_tab_at_1
language: rust
---

# save_primitive_tab_at

Write the primitive at `path` back to disk as JSON, commit

## Signature

```rust
pub(crate) fn save_primitive_tab_at(&mut self, path: &std::path::Path) -> anyhow::Result<()>
```

## Visibility

- `pub(crate)`

## Docstring

Write the primitive at `path` back to disk as JSON, commit
through the matching adapter (when the file lives under a
mounted `.snxlib/`), mark the tab clean, and ask the
`LibrarySet` to reload its cached copy so any open Component
Preview tabs see the new bytes.

Returns the save failure — serialize vs write, the target path,
and the underlying OS/serializer reason — so the caller can
surface WHY a save failed (Ctrl+S diagnostics, the Save-All-on-
exit dialog) instead of silently dropping it. Historically both
error legs emitted a `tracing::warn!` and returned `()`; those
events never reached `OxideLogger`, so a failed primitive save
was invisible in both stderr and the Messages panel.

## Source
Lines 401–519 in `crates/oxide-app/src/app/dispatch/library/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/app/dispatch/library/editor.md) |
| calls | [atomic_write](/crates/oxide-app/src/app/dispatch/library/recovery/atomic_write.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
