---
okf_version: "0.2"
type: Function
title: refresh_history_panel
description: "Recompute the History panel's target path from the active tab,"
resource: crates/oxide-app/src/app/runtime/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/history/refresh_history_panel
language: rust
---

# refresh_history_panel

Recompute the History panel's target path from the active tab,

## Signature

```rust
impl Oxide { pub(super) fn refresh_history_panel(&mut self) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Recompute the History panel's target path from the active tab,
bump the generation counter on change, and return a
`Task::perform` that loads the file's git history off the UI
thread. Called from [`Self::finish_update`] so every dispatch
path that ends with a `finish_update()` call refreshes the
panel automatically. Returns `Task::none()` when the target
hasn't changed since the last refresh.

## Source
Lines 11–103 in `crates/oxide-app/src/app/runtime/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/app/runtime/history.md) |
| calls | [resolve_history_target](/crates/oxide-app/src/app/runtime/history/resolve_history_target.md) |
| calls | [project_file_history](/crates/oxide-library/src/lib/project_file_history.md) |
