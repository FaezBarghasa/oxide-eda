---
okf_version: "0.2"
type: Class
title: HistoryEntry
description: "Plain-data view of one commit row, decoupled from"
resource: crates/oxide-widgets/src/history_pane.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/history_pane/HistoryEntry
language: rust
---

# HistoryEntry

Plain-data view of one commit row, decoupled from

## Signature

```rust
pub struct HistoryEntry
```

## Decorators

- `derive(Clone, Debug, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Plain-data view of one commit row, decoupled from
`oxide_library::HistoryEntry` so the widget crate doesn't pull
the library crate as a dep.

The library crate's [`oxide_library::HistoryEntry`] is the
canonical source — callers convert via `From<&_>` at the
boundary (see the `From` impl wired up by callers when both
crates are in scope).
[derive(Clone, Debug, PartialEq, Eq)]

## Methods

- `sha`
- `author_name`
- `author_email`
- `time`
- `subject`

## Source
Lines 41–53 in `crates/oxide-widgets/src/history_pane.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history_pane](/crates/oxide-widgets/src/history_pane.md) |
