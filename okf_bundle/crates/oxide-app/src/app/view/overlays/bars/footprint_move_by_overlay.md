---
okf_version: "0.2"
type: Function
title: footprint_move_by_overlay
description: "v0.14 — typed-delta \"Move Selection By X, Y…\" modal for the"
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/footprint_move_by_overlay
language: rust
---

# footprint_move_by_overlay

v0.14 — typed-delta "Move Selection By X, Y…" modal for the

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn footprint_move_by_overlay(&self) -> Vec<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

v0.14 — typed-delta "Move Selection By X, Y…" modal for the
footprint editor. A blocking dialog once open; pushes its dismiss
backdrop then the centered card.

## Source
Lines 349–383 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
| calls | [view_move_by_modal](/crates/oxide-app/src/library/editor/footprint/move_by_modal/view_move_by_modal.md) |
