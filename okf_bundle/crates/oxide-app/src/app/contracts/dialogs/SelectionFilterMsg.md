---
okf_version: "0.2"
type: Class
title: SelectionFilterMsg
description: Custom Selection Filter modal message family (ADR-0001 D3).
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/SelectionFilterMsg
language: rust
---

# SelectionFilterMsg

Custom Selection Filter modal message family (ADR-0001 D3).

## Signature

```rust
pub enum SelectionFilterMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Custom Selection Filter modal message family (ADR-0001 D3).
Namespaced under `Message::SelectionFilter` and routed to
`dispatch_selection_filter_message`. Drives the footprint editor's
selection-filter customization modal.
[derive(Debug, Clone)]

## Source
Lines 226–240 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
