---
okf_version: "0.2"
type: Function
title: every_active_bar_action_is_accounted_for
description: "Ratchet in the other direction: every `ActiveBarAction` must have an"
resource: crates/oxide-app/src/app/command/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/command/active_bar/every_active_bar_action_is_accounted_for
language: rust
---

# every_active_bar_action_is_accounted_for

Ratchet in the other direction: every `ActiveBarAction` must have an

## Signature

```rust
fn every_active_bar_action_is_accounted_for()
```

## Decorators

- `test`

## Docstring

Ratchet in the other direction: every `ActiveBarAction` must have an
id, be a parameterised family awaiting `CommandArgs`, or be mapped
elsewhere. Adding a variant without deciding which fails here rather
than silently leaving it unaddressable.

Counted rather than matched exhaustively because `ActiveBarAction`
is not `Hash` and the crate has no variant iterator; the total is
asserted explicitly so a new variant cannot slip past.
[test]

## Source
Lines 432–445 in `crates/oxide-app/src/app/command/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/app/command/active_bar.md) |
