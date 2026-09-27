---
okf_version: "0.2"
type: Function
title: status_bar
description: Status bar at the bottom
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/status_bar
language: rust
---

# status_bar

Status bar at the bottom

## Signature

```rust
pub fn status_bar(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Status bar at the bottom

## Source
Lines 124–138 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_symbol_status](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_status.md) |
| called_by | [view](/crates/oxide-app/src/status_bar/view.md) |
