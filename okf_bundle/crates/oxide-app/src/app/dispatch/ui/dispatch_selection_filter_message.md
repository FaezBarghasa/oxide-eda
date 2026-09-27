---
okf_version: "0.2"
type: Function
title: dispatch_selection_filter_message
description: "Custom Selection Filter modal handler (namespaced family,"
resource: crates/oxide-app/src/app/dispatch/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/ui/dispatch_selection_filter_message
language: rust
---

# dispatch_selection_filter_message

Custom Selection Filter modal handler (namespaced family,

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_selection_filter_message(
        &mut self,
        msg: SelectionFilterMsg,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Custom Selection Filter modal handler (namespaced family,
ADR-0001 D3). Drives the footprint editor's selection-filter
customization modal.

## Source
Lines 147–221 in `crates/oxide-app/src/app/dispatch/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/app/dispatch/ui.md) |
