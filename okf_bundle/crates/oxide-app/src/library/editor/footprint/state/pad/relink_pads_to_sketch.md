---
okf_version: "0.2"
type: Function
title: relink_pads_to_sketch
description: "Re-attach each pad's `sketch_entity_id` from the sketch itself, by"
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/relink_pads_to_sketch
language: rust
---

# relink_pads_to_sketch

Re-attach each pad's `sketch_entity_id` from the sketch itself, by

## Signature

```rust
pub(super) fn relink_pads_to_sketch(pads: &mut [EditorPad], fp: &oxide_library::Footprint)
```

## Visibility

- `pub(super)`

## Docstring

Re-attach each pad's `sketch_entity_id` from the sketch itself, by
matching the `PadAttr.number` the centre `Point` carries.

`EditorPad::from_pad` cannot restore it — the link has no home on
`Pad`, so every pad rebuilt from the primitive comes back with
`sketch_entity_id: None`. That is silent data loss on the next
edit, not a cosmetic gap: `mirror_move_pad_in_sketch` and
`mirror_delete_pad_from_sketch` both early-return on a `None` link,
so after a save + reopen a Pads-mode move left the pad's whole
outline stranded at its old position (the bake then emits copper
from the stranded geometry) and a Pads-mode delete left the outline
AND its `PadAttr`-carrying centre behind, resurrecting the deleted
pad on the next bake.

The sketch is the durable side of the link, so it is the side the
link is rebuilt from. Shared by [`super::FootprintEditorState::from_footprint`]
(open / reopen) and `refresh_pads_from_primitive` (post-bake
refresh) so the two loaders cannot drift apart.

# Pad numbers are NOT unique

Nothing in oxide enforces a unique pad number — the Properties
field takes any string and `next_pad_defaults.designator_override`
stamps one number onto every pad placed after it, which is how a
shared-designator row / thermal / shield pad set is authored. Each
such pad still mints its OWN `PadAttr`-bearing centre, so a plain
number → id map is last-wins and hands several pads the SAME
centre. Aliased that way, a Pads-mode delete of one pad runs the
delete mirror against ANOTHER pad's geometry and its `PadAttr`
ledger: that pad's whole outline goes, the pad itself stays in the
list, and since the bake resolves copper from the sketch its copper
silently vanishes from the export. A move aliases the same way.

So the number is only the first half of the key. Positions
disambiguate a collision — exactly, never by epsilon, and the pad
centre's `Point` is written from `pad.position_mm` verbatim. If a
collision survives that (two pads sharing a number AND a position),
the link is REFUSED: an unlinked pad makes both mirrors early-return,
which is the pre-fix #142 behaviour — stale geometry, but never
another pad's geometry destroyed.

## Source
Lines 642–681 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
| calls | [pos_key](/crates/oxide-app/src/library/editor/footprint/state/pad/pos_key.md) |
| calls | [pad_pos_key](/crates/oxide-app/src/library/editor/footprint/state/pad/pad_pos_key.md) |
| calls | [resolve_link](/crates/oxide-app/src/library/editor/footprint/state/pad/resolve_link.md) |
| called_by | [from_footprint](/crates/oxide-app/src/library/editor/footprint/state/mod/from_footprint.md) |
| called_by | [refresh_pads_from_primitive](/crates/oxide-app/src/library/editor/footprint/state/mod/refresh_pads_from_primitive.md) |
