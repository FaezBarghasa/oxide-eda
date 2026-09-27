---
okf_version: "0.2"
type: Function
title: os_close_request_proceeds_when_preferences_is_clean
description: "Control for the test above: the same OS close request on a CLEAN"
resource: crates/oxide-app/tests/regression/preferences_dirty_guard.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_dirty_guard/os_close_request_proceeds_when_preferences_is_clean
language: rust
---

# os_close_request_proceeds_when_preferences_is_clean

Control for the test above: the same OS close request on a CLEAN

## Signature

```rust
fn os_close_request_proceeds_when_preferences_is_clean()
```

## Decorators

- `test`

## Docstring

Control for the test above: the same OS close request on a CLEAN
Preferences window must proceed (a real `window::close` command), so the
guard is a dirty-only gate, not a Preferences-window-never-closes bug.
[test]

## Source
Lines 165–184 in `crates/oxide-app/tests/regression/preferences_dirty_guard.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_dirty_guard](/crates/oxide-app/tests/regression/preferences_dirty_guard.md) |
