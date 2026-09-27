---
okf_version: "0.2"
type: Function
title: mirror_about_own_vertical_axis
description: "Mirror every mirror-sensitive field about the pad's OWN"
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/mirror_about_own_vertical_axis_1
language: rust
---

# mirror_about_own_vertical_axis

Mirror every mirror-sensitive field about the pad's OWN

## Signature

```rust
pub fn mirror_about_own_vertical_axis(&mut self)
```

## Visibility

- `pub`

## Docstring

Mirror every mirror-sensitive field about the pad's OWN
vertical axis (local `x → -x`). This is what moving a pad to
the other side of the board does to its copper.

`oxide_bake::pad` consumes each of these verbatim with no
side-based mirroring of its own, so the stored data IS the
baked geometry. Mirroring only a subset bakes a shape that is
neither the front nor the back one — a Chamfered pad flipped
with its angle negated but its corner flags left alone keeps
the chamfer on the wrong corner and the part will not seat.
Every field that changes under `x → -x` therefore moves here
together, or none of them do.

Pad POSITION is deliberately untouched: this mirrors each pad
in place. Mirroring the layout about the footprint origin is a
different operation and does not exist yet.

## Source
Lines 280–298 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
