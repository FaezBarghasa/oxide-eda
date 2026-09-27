---
okf_version: "0.2"
type: Function
title: esc_in_the_main_window_still_leaves_a_detached_modal_alone
description: "[test]"
resource: crates/oxide-app/src/app/dispatch/escape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/escape/esc_in_the_main_window_still_leaves_a_detached_modal_alone
language: rust
---

# esc_in_the_main_window_still_leaves_a_detached_modal_alone

[test]

## Signature

```rust
fn esc_in_the_main_window_still_leaves_a_detached_modal_alone()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 272–294 in `crates/oxide-app/src/app/dispatch/escape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [escape](/crates/oxide-app/src/app/dispatch/escape.md) |
| calls | [quiet_app](/crates/oxide-app/src/app/dispatch/escape/quiet_app.md) |
| calls | [detach](/crates/oxide-app/src/app/dispatch/escape/detach.md) |
