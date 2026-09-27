---
okf_version: "0.2"
type: Function
title: arc_sweeps_through_mid
description: "Does the CCW span from `a0` to `a1` contain `am`?"
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/arc_sweeps_through_mid
language: rust
---

# arc_sweeps_through_mid

Does the CCW span from `a0` to `a1` contain `am`?

## Signature

```rust
pub fn arc_sweeps_through_mid(a0: f64, am: f64, a1: f64) -> bool
```

## Visibility

- `pub`

## Docstring

Does the CCW span from `a0` to `a1` contain `am`?

The rule that decides which of the two arcs through three clicked points
is the one the user meant. `pub` so the in-progress preview can ask the
same question the commit path asks — a preview that orders its endpoints
by a second copy of this rule can show the complementary arc to the one it
is about to create.

## Source
Lines 754–758 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| called_by | [draw_arc_preview](/crates/oxide-app/src/canvas/draw/previews/draw_arc_preview.md) |
| called_by | [build_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/snapshot/build_renderer_snapshot.md) |
