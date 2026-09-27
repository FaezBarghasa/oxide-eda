---
okf_version: "0.2"
type: Function
title: claim
description: "What `consumer` makes of this event."
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/claim_1
language: rust
---

# claim

What `consumer` makes of this event.

## Signature

```rust
fn claim(
        &self,
        consumer: InputConsumer,
        target: InputTarget,
        window: iced::window::Id,
        event: &keyboard::Event,
    ) -> Claim
```

## Docstring

What `consumer` makes of this event.

Exhaustive over [`InputConsumer`]: a new consumer will not compile
until it states its claim.

## Source
Lines 224–239 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
