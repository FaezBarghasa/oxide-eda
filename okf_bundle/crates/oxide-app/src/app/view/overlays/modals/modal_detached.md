---
okf_version: "0.2"
type: Function
title: modal_detached
description: "True while `modal` is showing in its own detached OS window rather"
resource: crates/oxide-app/src/app/view/overlays/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/modals/modal_detached
language: rust
---

# modal_detached

True while `modal` is showing in its own detached OS window rather

## Signature

```rust
impl Oxide { pub(crate) fn modal_detached(&self, modal: crate::app::state::ModalId) -> bool }
```

## Visibility

- `pub(crate)`

## Docstring

True while `modal` is showing in its own detached OS window rather
than as an in-window card. The detachable builders below skip
painting in that case, so the modal never renders in both windows
at once.

Visible to the whole `view` module since #547: `bars.rs` had two
hand-rolled copies of this predicate and a third place —
`has_blocking_modal` — that was missing it entirely.

`pub(crate)` since #555: the input router asks the same question
to decide which window a detachable modal's keys belong to.

## Source
Lines 139–143 in `crates/oxide-app/src/app/view/overlays/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/overlays/modals.md) |
