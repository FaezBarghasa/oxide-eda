---
okf_version: "0.2"
type: Function
title: re_docking_a_floating_panel_sends_it_to_its_home_region
description: "The floating panel's own dock button carries no drop target, so"
resource: crates/oxide-app/src/dock/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/state/re_docking_a_floating_panel_sends_it_to_its_home_region
language: rust
---

# re_docking_a_floating_panel_sends_it_to_its_home_region

The floating panel's own dock button carries no drop target, so

## Signature

```rust
fn re_docking_a_floating_panel_sends_it_to_its_home_region()
```

## Decorators

- `test`

## Docstring

The floating panel's own dock button carries no drop target, so
it used to send every kind to the right column.
[test]

## Source
Lines 492–500 in `crates/oxide-app/src/dock/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/dock/state.md) |
| calls | [floating](/crates/oxide-app/src/dock/state/floating.md) |
