---
okf_version: "0.2"
type: Function
title: history_loaded
description: "The completion message the async loader would deliver, tagged with"
resource: crates/oxide-app/tests/regression/history_load_failure.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/history_load_failure/history_loaded
language: rust
---

# history_loaded

The completion message the async loader would deliver, tagged with

## Signature

```rust
fn history_loaded(
    app: &Oxide,
    result: Result<Vec<oxide_widgets::HistoryEntry>, String>,
) -> Message
```

## Docstring

The completion message the async loader would deliver, tagged with
the generation the app is currently waiting on (a mismatched
generation is dropped as stale before any of this runs).

## Source
Lines 21–30 in `crates/oxide-app/tests/regression/history_load_failure.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history_load_failure](/crates/oxide-app/tests/regression/history_load_failure.md) |
| called_by | [a_failed_history_walk_reaches_the_messages_panel](/crates/oxide-app/tests/regression/history_load_failure/a_failed_history_walk_reaches_the_messages_panel.md) |
| called_by | [a_failed_history_walk_renders_as_an_error_not_an_empty_list](/crates/oxide-app/tests/regression/history_load_failure/a_failed_history_walk_renders_as_an_error_not_an_empty_list.md) |
| called_by | [a_successful_history_walk_is_still_ready](/crates/oxide-app/tests/regression/history_load_failure/a_successful_history_walk_is_still_ready.md) |
