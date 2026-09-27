---
okf_version: "0.2"
type: Function
title: format_relative
description: "Render `time` as a Slack/GitHub-style relative timestamp (\"12m"
resource: crates/oxide-widgets/src/history_pane.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/history_pane/format_relative
language: rust
---

# format_relative

Render `time` as a Slack/GitHub-style relative timestamp ("12m

## Signature

```rust
fn format_relative(time: DateTime<Utc>, now: DateTime<Utc>) -> String
```

## Docstring

Render `time` as a Slack/GitHub-style relative timestamp ("12m
ago", "3h ago", "5d ago", "2mo ago", "3y ago"). Future times
(clock skew, badly-set author date) collapse to "just now"
rather than a misleading negative figure.

## Source
Lines 167–188 in `crates/oxide-widgets/src/history_pane.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history_pane](/crates/oxide-widgets/src/history_pane.md) |
