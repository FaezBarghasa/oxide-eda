---
okf_version: "0.2"
type: Function
title: one_undo_after_a_rotate_restores_the_prior_sketch_geometry
description: "THE INVARIANT (c). The rotate now DROPS and re-mints the sidecar,"
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/one_undo_after_a_rotate_restores_the_prior_sketch_geometry
language: rust
---

# one_undo_after_a_rotate_restores_the_prior_sketch_geometry

THE INVARIANT (c). The rotate now DROPS and re-mints the sidecar,

## Signature

```rust
fn one_undo_after_a_rotate_restores_the_prior_sketch_geometry()
```

## Decorators

- `test`

## Docstring

THE INVARIANT (c). The rotate now DROPS and re-mints the sidecar,
which is a far larger mutation than moving four points. One Ctrl+Z
still has to put the sketch back exactly as it was, or the re-mint
is a one-way loss of the user's outline.

NOT A PROOF OF THE FIX, and do not read it as one. This test does
not go red when the re-mint is reverted: the behaviour it replaces
is a four-point corner move, which is trivially undoable, so it
passes either way. It is a FORWARD-LOOKING guard — it fails the day
someone makes the re-mint one-way — and it is kept for that.

Which test guards which mechanism, precisely — the same trap one
level down, since (b) was previously listed as a proof of the
re-mint and is not one:

- (a) guards THE RE-MINT on rotate. Reverting the
`remint_pad_geometry` call in `updates/active_bar.rs` to
`mirror_move_pad_in_sketch` turns it red on its own.
- (b) guards `attr::mirror_shape` — the editor-shape → `PadAttr`
mirror the flip needs. It does NOT discriminate the re-mint:
revert both `remint_pad_geometry` calls in `active_bar.rs` and it
stays green. It goes red only when `mirror_shape` is also
reverted, which is what the earlier whole-commit revert did.
- (d), (e), (g) and (h) each guard the re-mint at one further frame-
change site (Properties-panel rotation, the size / shape funnel,
the Sketch-mode edge drag, the Sketch-mode corner drag).
- (f) guards the sidecar TRANSLATION on the six move sites.
- `a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick`
guards the re-mint at (g) and (h) being an IN-PLACE one. It is red
for a plain drop-and-re-mint, which invalidates the id the pointer
is dragging.
- `sketch_centre_drag_carries_the_chamfer_anchor_with_it` proves
nothing — see its own comment.

Green tests are not proofs; each of the above was confirmed red with
its own mechanism reverted and green with it restored.
[test]

## Source
Lines 322–361 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [editor_with_minted_pad](/crates/oxide-app/tests/footprint_pad_remint/editor_with_minted_pad.md) |
| calls | [chamfered_repro_pad](/crates/oxide-app/tests/footprint_pad_remint/chamfered_repro_pad.md) |
| calls | [geometry_fingerprint](/crates/oxide-app/tests/footprint_pad_remint/geometry_fingerprint.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_remint/dispatch.md) |
