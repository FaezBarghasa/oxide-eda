---
okf_version: "0.2"
type: Function
title: mirror
description: "v0.22 Phase B1 + extension — Mirror tool. Pre-condition: a Line entity"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/mirror
language: rust
---

# mirror

v0.22 Phase B1 + extension — Mirror tool. Pre-condition: a Line entity

## Signature

```rust
fn mirror(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Docstring

v0.22 Phase B1 + extension — Mirror tool. Pre-condition: a Line entity
must already be selected via the Select tool; clicks while no Line is
selected are silent no-ops with a warning surfaced via
`solve_warnings`.

The picked entity's geometry is reflected across the selected Line and
a fresh entity is minted referencing mirrored copies of every Point it
touches. Each mirrored Point pair gets a `SymmetricAboutLine`
constraint so the solver maintains symmetry through subsequent edits
(drag the source and the mirror tracks it parametrically).

Scope: Points / Lines / Arcs / Circles. Mirrored Arcs flip `sweep_ccw`
because reflection inverts winding. Mirrored Circles re-use the source
radius (Circle's `radius` is a literal, not a referenced Point, so it
round-trips unchanged).

## Source
Lines 51–276 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/apply.md) |
