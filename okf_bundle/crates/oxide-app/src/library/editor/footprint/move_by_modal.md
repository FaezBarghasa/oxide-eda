---
okf_version: "0.2"
type: Module
title: move_by_modal
description: "v0.14 — \"Move Selection By X, Y…\" typed-delta modal for the"
resource: crates/oxide-app/src/library/editor/footprint/move_by_modal.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/move_by_modal
language: rust
---

# move_by_modal

v0.14 — "Move Selection By X, Y…" typed-delta modal for the

## Docstring

v0.14 — "Move Selection By X, Y…" typed-delta modal for the
footprint editor.

Replaces the plain one-grid-step nudge as the active-bar's primary
"Move Selection by X, Y…" action: the user types an exact (dx, dy)
mm offset instead of nudging by the active grid step. Confirm feeds
the parsed delta into the SAME proven
`footprint_nudge_selection` dispatcher helper (history snapshot +
`nudge_pads` + sketch mirror + primitive re-sync) that the one-step
nudge uses — see `app/dispatch/library.rs`.

Mounting site: `app/view/mod.rs::collect_overlays`, in the same
block that mounts the footprint active bar + its dropdown overlay,
gated by the `needs_overlay` predicate (`ed.state.move_by_modal
.is_some()`). A modal flag left out of `needs_overlay` renders as a
silent no-op from a non-canvas tab — see
`reference_overlay_predicate_gotcha` in project memory.

## Relationships

| Type | Target |
|------|--------|
| related | [view_move_by_modal](/crates/oxide-app/src/library/editor/footprint/move_by_modal/view_move_by_modal.md) |
| related | [close_x](/crates/oxide-app/src/library/editor/footprint/move_by_modal/close_x.md) |
| related | [secondary_button](/crates/oxide-app/src/library/editor/footprint/move_by_modal/secondary_button.md) |
| related | [primary_button](/crates/oxide-app/src/library/editor/footprint/move_by_modal/primary_button.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
