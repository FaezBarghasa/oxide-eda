---
okf_version: "0.2"
type: Function
title: format_relative_simple
description: Coarse relative-time helper. Mirrors what
resource: crates/oxide-app/src/panels/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/history/format_relative_simple
language: rust
---

# format_relative_simple

Coarse relative-time helper. Mirrors what

## Signature

```rust
fn format_relative_simple(time: chrono::DateTime<Utc>, now: chrono::DateTime<Utc>) -> String
```

## Docstring

Coarse relative-time helper. Mirrors what
`oxide_widgets::history_pane::format_relative` does but is
inlined here so the panel doesn't need to expose the widget's
internal helper.

## Source
Lines 254–277 in `crates/oxide-app/src/panels/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/panels/history.md) |
