---
okf_version: "0.2"
type: Function
title: every_modal_claims_escape
description: "Every *field of `OpenOverlays`* claims Esc with its own Cancel"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/every_modal_claims_escape
language: rust
---

# every_modal_claims_escape

Every *field of `OpenOverlays`* claims Esc with its own Cancel

## Signature

```rust
fn every_modal_claims_escape()
```

## Decorators

- `test`

## Docstring

Every *field of `OpenOverlays`* claims Esc with its own Cancel
message — NOT "every modal in the app". That gap is real and is
exactly what let #511 (the passive calculator) and #514 (~14 more
modals) ship with no rung: a modal only gets covered here once
someone remembers to add it as a field and a rung, and nothing
forces that. Documented modals that are deliberately NOT fields —
so NOT covered by this test — are: the command palette and the
keymap chord recorder (`palette_and_keymap_recorder_are_not_on_the_ladder`,
both swallow input earlier), the Edit Component Details modal
(dead behind `EDIT_MODAL_ENABLED = false`), and the per-browser
delete-confirm modal (its Cancel message needs a `library_path`
this `Copy` struct can't carry).
[test]

## Source
Lines 834–988 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
