---
okf_version: "0.2"
type: Module
title: history
description: Git history panel — right-dock surface that follows the active
resource: crates/oxide-app/src/panels/history.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/history
language: rust
---

# history

Git history panel — right-dock surface that follows the active

## Docstring

Git history panel — right-dock surface that follows the active
tab. Reuses [`oxide_widgets::history_pane`] to render the
actual cards. State is kept minimal: an active path resolved
from the active tab + the last loaded vec of entries + a
"loading" generation counter to discard stale async results.

Phase 0 (this module) wires the read-only view; the load path
is driven by `oxide_app`'s dispatcher via
[`crate::app::HistoryLoad`] (a `Message::HistoryLoaded` variant
threads each result back to the panel context with a generation
token, so a tab switch in flight discards any pending result).

## Relationships

| Type | Target |
|------|--------|
| related | [HistoryPanelState](/crates/oxide-app/src/panels/history/HistoryPanelState.md) |
| related | [HistoryRenderMode](/crates/oxide-app/src/panels/history/HistoryRenderMode.md) |
| related | [view_history](/crates/oxide-app/src/panels/history/view_history.md) |
| related | [message_card](/crates/oxide-app/src/panels/history/message_card.md) |
| related | [commit_card](/crates/oxide-app/src/panels/history/commit_card.md) |
| related | [short_sha](/crates/oxide-app/src/panels/history/short_sha.md) |
| related | [format_relative_simple](/crates/oxide-app/src/panels/history/format_relative_simple.md) |
| related | [working_tree_card](/crates/oxide-app/src/panels/history/working_tree_card.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
