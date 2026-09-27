---
okf_version: "0.2"
type: Class
title: SchematicCanvas
description: "The schematic `canvas::Program` — a per-frame *view* of the app state,"
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/SchematicCanvas
language: rust
---

# SchematicCanvas

The schematic `canvas::Program` — a per-frame *view* of the app state,

## Signature

```rust
pub struct SchematicCanvas
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

The schematic `canvas::Program` — a per-frame *view* of the app state,
not an owner of it.

#631 — this used to be one long-lived struct in `InteractionState` with
35 fields, about a dozen of them copies of `UiState` / `DocumentState`
data kept in sync across ~93 assignment sites with nothing enforcing
agreement. It is now constructed in `view` from two borrows: the
per-window [`CanvasSlot`] (the caches, camera and genuinely per-window
interaction state, which have no other home) and [`CanvasViewPrefs`]
(read from app state each frame). The library editors already had this
shape — `FootprintCanvas<'a>` and `SymbolCanvas<'a>` — so this is the
schematic canvas catching up, not a new pattern.

`Deref` to the slot is deliberate: it keeps every `self.selected`,
`self.ghost_symbol`, `self.camera()` in the draw and input modules
reading exactly as before, so the diff is the state that actually
moved rather than a mechanical prefix sweep over 200 field accesses.

## Methods

- `slot`
- `prefs`

## Source
Lines 321–324 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
