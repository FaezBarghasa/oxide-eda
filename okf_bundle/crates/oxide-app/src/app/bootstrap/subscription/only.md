---
okf_version: "0.2"
type: Function
title: only
description: Set one flag on an otherwise-closed set and resolve the Esc ladder.
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/only
language: rust
---

# only

Set one flag on an otherwise-closed set and resolve the Esc ladder.

## Signature

```rust
fn only(set: impl FnOnce(&mut OpenOverlays) -> Option<Message>
```

## Docstring

Set one flag on an otherwise-closed set and resolve the Esc ladder.

## Source
Lines 684–688 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
| called_by | [a_detached_modal_answers_escape_with_the_same_message_its_card_would](/crates/oxide-app/src/app/bootstrap/subscription/a_detached_modal_answers_escape_with_the_same_message_its_card_would.md) |
