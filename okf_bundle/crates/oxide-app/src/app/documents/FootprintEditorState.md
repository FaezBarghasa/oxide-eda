---
okf_version: "0.2"
type: Class
title: FootprintEditorState
description: "Per-tab state for an open `.snxfpt` document. Mirrors the"
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/FootprintEditorState
language: rust
---

# FootprintEditorState

Per-tab state for an open `.snxfpt` document. Mirrors the

## Signature

```rust
pub struct FootprintEditorState
```

## Decorators

- `derive(Debug)`

## Visibility

- `pub`

## Docstring

Per-tab state for an open `.snxfpt` document. Mirrors the
footprint-editing fields on `ComponentEditorState` but keyed by
file path. Reuses the existing
[`crate::library::editor::footprint::canvas::FootprintCanvas`] +
[`crate::library::editor::footprint::state::FootprintEditorState`]
so the behaviour matches the in-Component Editor experience verbatim.

v0.18.6 — mirrors [`SymbolEditorState`]: the editor backs a multi-
footprint container and dispatches every mutation through
`file.footprints[active_idx]`. Saves preserve `file_uuid` and any
future siblings instead of minting a fresh single-footprint
envelope.
[derive(Debug)]

## Methods

- `path`
- `file`
- `active_idx`
- `panel_selected_idx`
- `state`
- `canvas_cache`
- `dirty`
- `history`
- `redo`

## Source
Lines 404–434 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
