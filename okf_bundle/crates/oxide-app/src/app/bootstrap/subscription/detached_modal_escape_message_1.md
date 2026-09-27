---
okf_version: "0.2"
type: Function
title: detached_modal_escape_message
description: "What Esc does inside `modal`'s own detached window: exactly what"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/detached_modal_escape_message_1
language: rust
---

# detached_modal_escape_message

What Esc does inside `modal`'s own detached window: exactly what

## Signature

```rust
pub(crate) fn detached_modal_escape_message(
        &self,
        modal: crate::app::state::ModalId,
    ) -> Option<Message>
```

## Visibility

- `pub(crate)`

## Docstring

What Esc does inside `modal`'s own detached window: exactly what
Esc over that modal's in-window card would do, and nothing else
(#547).

The ladder is deliberately NOT walked here. It ranks overlays by
where they paint **in the main window**, and none of that stack
covers a separate OS window — running it would let a dialog in
another window outrank the one the user is actually typing into.
So the answer is a single rung, looked up by id.

`modal`'s own detachment is passed to `open_overlays` so its flag
reads open: the flag is normally forced false *because* the
in-window card is not painted, which is the right answer for an
Esc in the main window and the wrong one here.

`None` means that modal has no Esc at all — either no `OverlayId`
(Move Selection, the net-colour palette, the parameter manager)
or an `OverlayId` whose rung is `None` by design. Its in-window
card ignores Esc too, so this stays consistent with it rather than
inventing a dismissal only the detached form has.

## Source
Lines 404–410 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
| calls | [modal_overlay_id](/crates/oxide-app/src/app/bootstrap/subscription/modal_overlay_id.md) |
