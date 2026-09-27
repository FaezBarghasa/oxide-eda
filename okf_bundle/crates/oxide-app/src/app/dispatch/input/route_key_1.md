---
okf_version: "0.2"
type: Function
title: route_key
description: "Walk [`CLAIM_ORDER`] and return the one message this event earns,"
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/route_key_1
language: rust
---

# route_key

Walk [`CLAIM_ORDER`] and return the one message this event earns,

## Signature

```rust
fn route_key(&self, window: iced::window::Id, event: &keyboard::Event) -> Option<Message>
```

## Docstring

Walk [`CLAIM_ORDER`] and return the one message this event earns,
or `None` when it was swallowed or nobody wanted it.

`&self` on purpose: this is the testable heart. Build a `Oxide`,
set state, feed an event, assert the message — none of which was
possible while these branches lived in a subscription closure.

## Source
Lines 208–218 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
