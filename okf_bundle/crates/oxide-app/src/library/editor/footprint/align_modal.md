---
okf_version: "0.2"
type: Module
title: align_modal
description: "#370 — \"Align…\" dialog for the footprint editor."
resource: crates/oxide-app/src/library/editor/footprint/align_modal.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/align_modal
language: rust
---

# align_modal

#370 — "Align…" dialog for the footprint editor.

## Docstring

#370 — "Align…" dialog for the footprint editor.

The generic Align ▸ "Align…" row used to be a `coming soon` stub;
this modal gives it a real per-axis dialog. It is a pure UI shell
over the EXISTING [`AlignOp`] variants — it introduces no new
geometry. The user picks at most one horizontal op and at most one
vertical op; Confirm applies both chosen axes under a SINGLE undo
snapshot (the handler in `updates::active_bar` owns that), so the
whole confirm is one undo step even when both axes move pads.

Structurally a sibling of `move_by_modal.rs` (the proven template):
a `view_align_modal` returning `Option<Element>` that yields `None`
when the modal is closed, all spacing/widths derived from the
`FONT_SIZE` constant, and `ti(tokens.…)` for every colour.

Mounting site: `app/view/mod.rs::collect_overlays`, alongside the
Move-By modal, gated by the `needs_overlay` predicate
(`ed.state.align_modal.is_some()`). A modal flag left out of
`needs_overlay` renders as a silent no-op from a non-canvas tab —
the same trap `move_by_modal.rs` documents.

## Relationships

| Type | Target |
|------|--------|
| related | [view_align_modal](/crates/oxide-app/src/library/editor/footprint/align_modal/view_align_modal.md) |
| related | [cancel_msg](/crates/oxide-app/src/library/editor/footprint/align_modal/cancel_msg.md) |
| related | [option_chip](/crates/oxide-app/src/library/editor/footprint/align_modal/option_chip.md) |
| related | [close_x](/crates/oxide-app/src/library/editor/footprint/align_modal/close_x.md) |
| related | [secondary_button](/crates/oxide-app/src/library/editor/footprint/align_modal/secondary_button.md) |
| related | [primary_button](/crates/oxide-app/src/library/editor/footprint/align_modal/primary_button.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
