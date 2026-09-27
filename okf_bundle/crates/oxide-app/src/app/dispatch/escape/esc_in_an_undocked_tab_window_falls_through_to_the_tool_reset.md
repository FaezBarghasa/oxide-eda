---
okf_version: "0.2"
type: Function
title: esc_in_an_undocked_tab_window_falls_through_to_the_tool_reset
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
concept_id: crates/oxide-app/src/app/dispatch/escape/esc_in_an_undocked_tab_window_falls_through_to_the_tool_reset
language: rust
---

# esc_in_an_undocked_tab_window_falls_through_to_the_tool_reset

[test]

## Signature

```rust
fn esc_in_an_undocked_tab_window_falls_through_to_the_tool_reset()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 471–490 in `crates/oxide-app/src/app/dispatch/escape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [escape](/crates/oxide-app/src/app/dispatch/escape.md) |
| calls | [quiet_app](/crates/oxide-app/src/app/dispatch/escape/quiet_app.md) |
| calls | [open_window](/crates/oxide-app/src/app/dispatch/escape/open_window.md) |
