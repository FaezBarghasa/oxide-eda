---
okf_version: "0.2"
type: Function
title: refresh_submenu_hover_state
description: Recompute the submenu close timer from the current hover zone
resource: crates/oxide-app/src/app/dispatch/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/overlay/refresh_submenu_hover_state_1
language: rust
---

# refresh_submenu_hover_state

Recompute the submenu close timer from the current hover zone

## Signature

```rust
fn refresh_submenu_hover_state(&mut self)
```

## Docstring

Recompute the submenu close timer from the current hover zone
booleans. If either the launcher row or the submenu panel is
hovered the close timer is cancelled; once *both* are clear and
a submenu is actually open, we arm the 150 ms delay. Called
after every hover-zone change so the close timer state never
contradicts the live hover flags.

## Source
Lines 521–531 in `crates/oxide-app/src/app/dispatch/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/app/dispatch/overlay.md) |
