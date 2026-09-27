---
okf_version: "0.2"
type: Class
title: HistoryEntry
description: One entry in the per-primitive git history feed.
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/HistoryEntry
language: rust
---

# HistoryEntry

One entry in the per-primitive git history feed.

## Signature

```rust
pub struct HistoryEntry
```

## Decorators

- `derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

One entry in the per-primitive git history feed.

Per `v0.9-snxlib-as-file-plan.md` §3 ("History panel inside the
per-primitive editor"), the SCH Library / Footprint / Sim editors
and the Library Browser tab all bind a [`HistoryEntry`] list to the
shared `oxide_widgets::history_pane::HistoryPane` widget. Stage 17
scaffolds the API + data shape; later stages layer the graph lane,
diff stats, and revert/reset affordances on top.

`parent_shas`, `files_changed`, `additions`, `deletions` are kept on
the struct so future polish stages (lazy diff stats, merge-graph
rendering) can land without a schema bump. The scaffold's
[`LocalGitAdapter`](crate::adapters::local_git::LocalGitAdapter)
implementation only fills `sha`, `author_name`, `author_email`,
`time`, `subject`, `body`, and `parent_shas`; the rest stay empty
until lazy-diff support arrives.
[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `sha`
- `author_name`
- `author_email`
- `time`
- `subject`
- `body`
- `parent_shas`
- `files_changed`
- `additions`
- `deletions`

## Source
Lines 104–133 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
