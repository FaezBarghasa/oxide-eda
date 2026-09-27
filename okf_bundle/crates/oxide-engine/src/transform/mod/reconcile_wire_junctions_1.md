---
okf_version: "0.2"
type: Function
title: reconcile_wire_junctions
description: "Mint any junction dots the sheet now needs because wires in `items`"
resource: crates/oxide-engine/src/transform/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/transform/mod/reconcile_wire_junctions_1
language: rust
---

# reconcile_wire_junctions

Mint any junction dots the sheet now needs because wires in `items`

## Signature

```rust
pub(super) fn reconcile_wire_junctions(&mut self, items: &[SelectedItem]) -> bool
```

## Visibility

- `pub(super)`

## Docstring

Mint any junction dots the sheet now needs because wires in `items`
changed shape, then drop any *minted* dot geometry no longer
justifies. Returns `true` when at least one dot was added or removed.

Move / rotate / mirror mutate existing wire coordinates; delete and
place-wire change which wires exist. All five used to reconcile
nothing beyond place-wire's own ad hoc mint call — dragging a stub
onto a trunk's interior left a junction-less T, which the netlist
reads as disconnected (issue #107), so the connection was silently
lost exactly as in issue #402's draw case. Every command that
changes wire geometry — place, move, rotate, mirror, delete —
routes through here now.

This used to be add-only: a dot left stale by geometry moving *apart*
was left in place on the theory that it was still correct as long as
it still sat on two wires. That theory doesn't hold — two wires can
come to sit on the same point by merely *crossing*, with neither one
terminating there, and geometric "on two wires" does not distinguish
that from a real T. A stale dot from a T that lost its stub then
re-asserts a connection on the first unrelated wire dragged across the
same point, silently merging two nets the user never connected (issue
#422). So every minted dot is re-validated here on every wire-geometry
command and removed once [`autoplace::wire_meeting_justifies_junction`]
no longer holds for it. A user-placed dot (`Junction::minted == false`)
is user data and is never considered for removal.

## Source
Lines 154–173 in `crates/oxide-engine/src/transform/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-engine/src/transform/mod.md) |
| calls | [junctions_for_wire](/crates/oxide-engine/src/transform/autoplace/junctions_for_wire.md) |
