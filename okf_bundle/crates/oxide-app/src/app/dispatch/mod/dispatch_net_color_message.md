---
okf_version: "0.2"
type: Function
title: dispatch_net_color_message
description: "Per-net colour override + F5-palette handler (namespaced family,"
resource: crates/oxide-app/src/app/dispatch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/mod/dispatch_net_color_message
language: rust
---

# dispatch_net_color_message

Per-net colour override + F5-palette handler (namespaced family,

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_net_color_message(&mut self, msg: NetColorMsg) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Per-net colour override + F5-palette handler (namespaced family,
ADR-0001 D3).

## Source
Lines 565–622 in `crates/oxide-app/src/app/dispatch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dispatch](/crates/oxide-app/src/app/dispatch/mod.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
