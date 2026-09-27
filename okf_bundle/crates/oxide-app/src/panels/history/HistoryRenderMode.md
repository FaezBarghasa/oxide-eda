---
okf_version: "0.2"
type: Class
title: HistoryRenderMode
description: "What the panel should render when `entries` is non-empty"
resource: crates/oxide-app/src/panels/history.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/history/HistoryRenderMode
language: rust
---

# HistoryRenderMode

What the panel should render when `entries` is non-empty

## Signature

```rust
pub enum HistoryRenderMode
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Default)`

## Visibility

- `pub`

## Docstring

What the panel should render when `entries` is non-empty
doesn't unambiguously cover the case (e.g. the file isn't even
version-controlled).
[derive(Debug, Clone, PartialEq, Eq, Default)]

## Source
Lines 55–77 in `crates/oxide-app/src/panels/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/panels/history.md) |
