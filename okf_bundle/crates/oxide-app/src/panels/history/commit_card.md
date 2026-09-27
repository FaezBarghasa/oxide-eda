---
okf_version: "0.2"
type: Function
title: commit_card
description: "v0.22 Phase 8.5 — Commit-row card with a \"Restore this version\""
resource: crates/oxide-app/src/panels/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/history/commit_card
language: rust
---

# commit_card

v0.22 Phase 8.5 — Commit-row card with a "Restore this version"

## Signature

```rust
fn commit_card(
    entry: &'a oxide_widgets::HistoryEntry,
    now: chrono::DateTime<Utc>,
    primary: Color,
    muted: Color,
    border_c: Color,
    bg: Option<iced::Background>,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Docstring

v0.22 Phase 8.5 — Commit-row card with a "Restore this version"
button. Same visual shape as `oxide_widgets::history_pane`'s
cards (author + relative time + subject + short SHA) plus a
muted button on the bottom that fires
`PanelMsg::HistoryRestoreClicked { sha }` on press. The handler
runs `LocalGitProjectAdapter::restore_at` against the active
tab's owning project.

## Source
Lines 182–244 in `crates/oxide-app/src/panels/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/panels/history.md) |
| called_by | [view_history](/crates/oxide-app/src/panels/history/view_history.md) |
