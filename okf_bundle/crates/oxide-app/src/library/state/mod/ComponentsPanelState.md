---
okf_version: "0.2"
type: Class
title: ComponentsPanelState
description: Per-source collapse + filter state for the Components Panel.
resource: crates/oxide-app/src/library/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/mod/ComponentsPanelState
language: rust
---

# ComponentsPanelState

Per-source collapse + filter state for the Components Panel.

## Signature

```rust
pub struct ComponentsPanelState
```

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

Per-source collapse + filter state for the Components Panel.
Persists across panel re-renders but not across app restarts —
cheap session-scoped UI flags.
[derive(Debug, Clone, Default)]

## Methods

- `collapsed_project`
- `collapsed_installed`
- `collapsed_global`
- `filter`

## Source
Lines 549–561 in `crates/oxide-app/src/library/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/state/mod.md) |
