---
okf_version: "0.2"
type: Function
title: active_footprint_editor
description: "v0.16.3 — sibling of [`Self::active_symbol_editor_mut`] for"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/active_footprint_editor_1
language: rust
---

# active_footprint_editor

v0.16.3 — sibling of [`Self::active_symbol_editor_mut`] for

## Signature

```rust
pub(crate) fn active_footprint_editor(&self) -> Option<&crate::app::FootprintEditorState>
```

## Visibility

- `pub(crate)`

## Docstring

v0.16.3 — sibling of [`Self::active_symbol_editor_mut`] for
`.snxfpt` editor tabs. Drives the Properties-panel pad-defaults
form so it can mutate `next_pad_defaults` without round-
tripping through `LibraryMessage::PrimitiveEditorEvent`.
Read-only sibling of [`active_footprint_editor_mut`].
v0.18.11 — used by the Grid Properties modal open handler
to seed the dialog buffers from the live snap step.

## Source
Lines 20–30 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.md) |
