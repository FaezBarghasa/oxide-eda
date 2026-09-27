---
okf_version: "0.2"
type: Function
title: show_panel_reveals_a_panel_hidden_behind_another_tab
description: "The papercut behind the \"open X did nothing\" half of #641:"
resource: crates/oxide-app/src/dock/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/state/show_panel_reveals_a_panel_hidden_behind_another_tab
language: rust
---

# show_panel_reveals_a_panel_hidden_behind_another_tab

The papercut behind the "open X did nothing" half of #641:

## Signature

```rust
fn show_panel_reveals_a_panel_hidden_behind_another_tab()
```

## Decorators

- `test`

## Docstring

The papercut behind the "open X did nothing" half of #641:
X was already docked, just behind another tab.
[test]

## Source
Lines 422–437 in `crates/oxide-app/src/dock/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/dock/state.md) |
