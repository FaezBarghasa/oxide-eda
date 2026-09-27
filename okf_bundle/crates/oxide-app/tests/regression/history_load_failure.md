---
okf_version: "0.2"
type: Module
title: history_load_failure
description: "#599 — a git history walk that failed is not \"no commits yet\"."
resource: crates/oxide-app/tests/regression/history_load_failure.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/history_load_failure
language: rust
---

# history_load_failure

#599 — a git history walk that failed is not "no commits yet".

## Docstring

#599 — a git history walk that failed is not "no commits yet".

`Message::HistoryLoaded` carries `Result<Vec<HistoryEntry>, String>`
and used to be `unwrap_or_default()`-ed straight into a `Ready` mode
with an empty entry list — which the History panel renders as the
"No history yet." card. A corrupt object or an unreadable `.git/`
was therefore presented as "this file has no commits", and "Restore
this version" silently disappeared. The panel already keeps `NoRepo`
separate from "no commits yet" for the same reason.

## Relationships

| Type | Target |
|------|--------|
| related | [history_loaded](/crates/oxide-app/tests/regression/history_load_failure/history_loaded.md) |
| related | [a_failed_history_walk_renders_as_an_error_not_an_empty_list](/crates/oxide-app/tests/regression/history_load_failure/a_failed_history_walk_renders_as_an_error_not_an_empty_list.md) |
| related | [a_successful_history_walk_is_still_ready](/crates/oxide-app/tests/regression/history_load_failure/a_successful_history_walk_is_still_ready.md) |
| related | [a_failed_history_walk_reaches_the_messages_panel](/crates/oxide-app/tests/regression/history_load_failure/a_failed_history_walk_reaches_the_messages_panel.md) |
