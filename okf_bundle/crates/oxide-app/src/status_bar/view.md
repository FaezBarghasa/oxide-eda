---
okf_version: "0.2"
type: Function
title: view
resource: crates/oxide-app/src/status_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/status_bar/view
language: rust
---

# view

## Signature

```rust
pub fn view(
    x: f64,
    y: f64,
    grid_visible: bool,
    snap_enabled: bool,
    zoom: f64,
    unit: Unit,
    tool: &Tool,
    grid_size_mm: f32,
    selected: &[SelectedItem],
    tokens: &ThemeTokens,
    pending_git_commits: usize,
) -> Element<'a, StatusBarRequest>
```

## Type Parameters

- `'a`

## Decorators

- `expect(
    clippy::too_many_arguments,
    reason = "11 arguments: the status bar reads eleven independent slices of app state and owns none of them"
)`

## Visibility

- `pub`

## Source
Lines 57–156 in `crates/oxide-app/src/status_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [status_bar](/crates/oxide-app/src/status_bar.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [selection_summary](/crates/oxide-app/src/status_bar/selection_summary.md) |
| calls | [status_bar](/crates/oxide-app/src/styles/status_bar.md) |
