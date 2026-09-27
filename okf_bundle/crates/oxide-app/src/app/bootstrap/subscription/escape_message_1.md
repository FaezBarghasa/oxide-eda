---
okf_version: "0.2"
type: Function
title: escape_message
description: "The Esc ladder: which modal a bare Esc dismisses. `None` means no"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/escape_message_1
language: rust
---

# escape_message

The Esc ladder: which modal a bare Esc dismisses. `None` means no

## Signature

```rust
fn escape_message(&self) -> Option<Message>
```

## Docstring

The Esc ladder: which modal a bare Esc dismisses. `None` means no
modal claimed the key, so the caller falls through to
`Message::EscapePressed` (the tool reset).

**Ordering is not decided here.** It is read off
[`PAINT_ORDER`](crate::app::view::overlay_id::PAINT_ORDER) walked
BACKWARD — whatever paints on top wins Esc — so a reorder happens
in exactly one place and reaches the painter and this ladder at
once. Until #535 part 2 this was 25 hand-written `if`s that a doc
comment asked you to keep in the exact reverse of a list in
another file; every ordering bug found across #514's review rounds
was that request not honoured in one spot, and patching the pairs
one at a time kept reintroducing new violations elsewhere.

[`visible`](crate::app::view::overlay_id::visible) supplies the
blocking-modal cutoff, so the "resolve only within the visible
five" guard is no longer written out here either — it falls out of
walking a shorter slice.

What this still does NOT quantify over is every modal in the app —
only over *fields of `OpenOverlays`*. That gap is real (see
`every_modal_claims_escape` at the foot of this file) and is
exactly what let #511 (the passive calculator) and #514 (~14 more
modals) ship with no rung at all. A field that IS here and has no
rung is a live bug: Esc falls through and silently resets the
active canvas tool *behind* the open modal.
`every_ladder_field_claims_escape` fails if a new field arrives
without one.

## Source
Lines 121–126 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
| calls | [visible](/crates/oxide-app/src/app/view/overlay_id/visible.md) |
