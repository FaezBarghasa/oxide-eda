---
okf_version: "0.2"
type: Class
title: HistoryPanelState
description: In-flight render state for the History panel. Owned by
resource: crates/oxide-app/src/panels/history.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/history/HistoryPanelState
language: rust
---

# HistoryPanelState

In-flight render state for the History panel. Owned by

## Signature

```rust
pub struct HistoryPanelState
```

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

In-flight render state for the History panel. Owned by
[`crate::app::DocumentState`] and projected into
[`super::PanelContext`] each refresh.
[derive(Debug, Clone, Default)]

## Methods

- `generation`
- `active_path`
- `loading`
- `entries`
- `mode`
- `dirty`

## Source
Lines 24–49 in `crates/oxide-app/src/panels/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/panels/history.md) |
