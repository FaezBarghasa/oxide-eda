---
okf_version: "0.2"
type: Function
title: an_undocked_window_does_not_inherit_the_main_windows_editor_contexts
description: "The #559 bug, stated as a contrast: the SAME app state resolves to"
resource: crates/oxide-app/src/app/dispatch/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/keymap/an_undocked_window_does_not_inherit_the_main_windows_editor_contexts
language: rust
---

# an_undocked_window_does_not_inherit_the_main_windows_editor_contexts

The #559 bug, stated as a contrast: the SAME app state resolves to

## Signature

```rust
fn an_undocked_window_does_not_inherit_the_main_windows_editor_contexts()
```

## Decorators

- `test`

## Docstring

The #559 bug, stated as a contrast: the SAME app state resolves to
different contexts depending on which window typed the stroke.
Before this, both answers were the main window's.
[test]

## Source
Lines 202–219 in `crates/oxide-app/src/app/dispatch/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/app/dispatch/keymap.md) |
| calls | [app_with_footprint_tab](/crates/oxide-app/src/app/dispatch/keymap/app_with_footprint_tab.md) |
| calls | [undocked](/crates/oxide-app/src/app/dispatch/keymap/undocked.md) |
