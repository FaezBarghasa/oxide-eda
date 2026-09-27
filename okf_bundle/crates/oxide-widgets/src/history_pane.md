---
okf_version: "0.2"
type: Module
title: history_pane
description: Reusable per-primitive git history pane.
resource: crates/oxide-widgets/src/history_pane.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/history_pane
language: rust
---

# history_pane

Reusable per-primitive git history pane.

## Docstring

Reusable per-primitive git history pane.

Per `v0.9-snxlib-as-file-plan.md` §3 ("History panel inside the
per-primitive editor"), every primitive editor (SCH Library,
Footprint, Sim) and the Library Browser tab binds the same
widget to its current selection. Stage 17 of that plan asks for
a *scaffold*: a simple text-only column of cards listing
commits, no graph lane / diff tabs / filter UI / right-click
menu yet — all of those are follow-up polish stages.

The widget is a **builder function** rather than a custom
`iced::Widget` impl: it returns an `Element` composed from stock
`column!` / `text` / `container` primitives so the shape is easy
to extend incrementally as later stages add affordances.

The struct definition lives in `oxide_library::HistoryEntry` so
the trait method that produces it (`LibraryAdapter::history`)
and the widget that consumes it agree on the data shape without
crossing a circular dep — `oxide-widgets` depends on
`oxide-types` only, so we re-declare the *shape* of an entry
locally as [`HistoryEntry`] and let callers convert at the
boundary. Same reason `tab_pill::TabPillStyle` exists separate
from any "real" tab type.

## Relationships

| Type | Target |
|------|--------|
| related | [HistoryEntry](/crates/oxide-widgets/src/history_pane/HistoryEntry.md) |
| related | [history_pane](/crates/oxide-widgets/src/history_pane/history_pane.md) |
| related | [empty_pane](/crates/oxide-widgets/src/history_pane/empty_pane.md) |
| related | [short_sha](/crates/oxide-widgets/src/history_pane/short_sha.md) |
| related | [format_relative](/crates/oxide-widgets/src/history_pane/format_relative.md) |
| related | [relative_time_buckets](/crates/oxide-widgets/src/history_pane/relative_time_buckets.md) |
| related | [short_sha_trims_to_seven](/crates/oxide-widgets/src/history_pane/short_sha_trims_to_seven.md) |
| related | [future_time_is_just_now](/crates/oxide-widgets/src/history_pane/future_time_is_just_now.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
