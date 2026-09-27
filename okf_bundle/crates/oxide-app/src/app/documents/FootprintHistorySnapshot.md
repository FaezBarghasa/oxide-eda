---
okf_version: "0.2"
type: Class
title: FootprintHistorySnapshot
description: v0.24 Phase 1 (Track B) — coarse-grained undo snapshot. Captures
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/FootprintHistorySnapshot
language: rust
---

# FootprintHistorySnapshot

v0.24 Phase 1 (Track B) — coarse-grained undo snapshot. Captures

## Signature

```rust
pub struct FootprintHistorySnapshot
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

v0.24 Phase 1 (Track B) — coarse-grained undo snapshot. Captures
the canonical state needed to roll back any footprint edit:
- the full multi-footprint `FootprintFile` (typically <50 KB of
in-memory representation),
- the active footprint index,
- the canvas-side `EditorPad` cache and its selection state,
- sketch selection state.

Stored by-value (not by-Arc) so undo/redo never aliases the live
state. Memory cost: ~5 KB × `HISTORY_DEPTH` = ~500 KB per editor.
Acceptable for v0.24; fine-grained inverse ops can replace this
later if memory becomes load-bearing.
[derive(Debug, Clone)]

## Methods

- `file`
- `active_idx`
- `pads`
- `selected_pad`
- `selected_sketch`
- `selected_sketch_secondary`

## Source
Lines 449–456 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
