---
okf_version: "0.2"
type: Class
title: UiMsg
description: UI / chrome message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/messages.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/contracts/messages/UiMsg
language: rust
---

# UiMsg

UI / chrome message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum UiMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

UI / chrome message family (ADR-0001 D3). Namespaced under
`Message::Ui` and routed to `dispatch_ui_message`. Groups theme,
unit + grid toggles, the footprint grid picker, layout drag, main-
window resize, and the status-bar request enum. Canvas events keep
their own top-level `Message::CanvasEvent(…)` variants.
[derive(Debug, Clone)]

## Methods

- `window`
- `stroke`

## Source
Lines 199–249 in `crates/oxide-app/src/app/contracts/messages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/app/contracts/messages.md) |
