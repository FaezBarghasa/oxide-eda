---
okf_version: "0.2"
type: Function
title: reveal_tab
description: "Make the tab at `idx` the active one in `position` and expand"
resource: crates/oxide-app/src/dock/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/state/reveal_tab_1
language: rust
---

# reveal_tab

Make the tab at `idx` the active one in `position` and expand

## Signature

```rust
fn reveal_tab(&mut self, position: PanelPosition, idx: usize)
```

## Docstring

Make the tab at `idx` the active one in `position` and expand
the region if the user had collapsed it. Opening a panel that
is docked behind another tab has to do this — otherwise "open
Signal" looks like it did nothing.

## Source
Lines 110–116 in `crates/oxide-app/src/dock/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/dock/state.md) |
