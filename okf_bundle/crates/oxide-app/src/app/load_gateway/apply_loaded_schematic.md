---
okf_version: "0.2"
type: Function
title: apply_loaded_schematic
description: Refresh the canvas and panel context against the active schematic
resource: crates/oxide-app/src/app/load_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/load_gateway/apply_loaded_schematic
language: rust
---

# apply_loaded_schematic

Refresh the canvas and panel context against the active schematic

## Signature

```rust
impl Oxide { pub(crate) fn apply_loaded_schematic(
        &mut self,
        schematic: Option<SchematicSheet>,
        clear_bg_cache: bool,
        fit_to_paper: bool,
        refresh_panel_ctx: bool,
    ) }
```

## Visibility

- `pub(crate)`

## Docstring

Refresh the canvas and panel context against the active schematic
engine, optionally installing `schematic` as that engine's document
first.

There is no "commit to the active tab" step: since the parked-session
refactor, schematic tabs keep their document in
`document_state.engines` (keyed by path) and carry
`cached_document: None`. `TabDocument` has no `Schematic` variant to
write back to, and dirty tracking runs through
`with_active_schematic_session_mut` in the mutation gateway.

## Source
Lines 384–414 in `crates/oxide-app/src/app/load_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [load_gateway](/crates/oxide-app/src/app/load_gateway.md) |
