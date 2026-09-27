---
okf_version: "0.2"
type: Function
title: footprint_align_overlay
description: "#370 — \"Align…\" dialog for the footprint editor. Mounted at the"
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/footprint_align_overlay
language: rust
---

# footprint_align_overlay

#370 — "Align…" dialog for the footprint editor. Mounted at the

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn footprint_align_overlay(&self) -> Vec<Element<'_, Message>> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

#370 — "Align…" dialog for the footprint editor. Mounted at the
same overlay layer as the Move-By modal (a blocking dialog once
open): a dismiss backdrop that routes to `AlignCancel`, then the
centered card.

## Source
Lines 389–423 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
| calls | [view_align_modal](/crates/oxide-app/src/library/editor/footprint/align_modal/view_align_modal.md) |
