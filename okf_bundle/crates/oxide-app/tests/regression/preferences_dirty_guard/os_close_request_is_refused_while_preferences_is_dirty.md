---
okf_version: "0.2"
type: Function
title: os_close_request_is_refused_while_preferences_is_dirty
description: "Finding 2 (data-loss): an OS close request (Alt+F4 / native ✕) on a"
resource: crates/oxide-app/tests/regression/preferences_dirty_guard.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_dirty_guard/os_close_request_is_refused_while_preferences_is_dirty
language: rust
---

# os_close_request_is_refused_while_preferences_is_dirty

Finding 2 (data-loss): an OS close request (Alt+F4 / native ✕) on a

## Signature

```rust
fn os_close_request_is_refused_while_preferences_is_dirty()
```

## Decorators

- `test`

## Docstring

Finding 2 (data-loss): an OS close request (Alt+F4 / native ✕) on a
dirty, detached Preferences window must be refused rather than silently
closing (which would let unsaved edits vanish with no confirmation and
no way back). `Task::units()` is iced's own public count of the actions a
`Task` carries — `Task::none()` (refused) is 0 units, `window::close(id)`
(proceeding) is 1 — so this observes the *actual* returned command
without needing the iced runtime.
[test]

## Source
Lines 131–159 in `crates/oxide-app/tests/regression/preferences_dirty_guard.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_dirty_guard](/crates/oxide-app/tests/regression/preferences_dirty_guard.md) |
