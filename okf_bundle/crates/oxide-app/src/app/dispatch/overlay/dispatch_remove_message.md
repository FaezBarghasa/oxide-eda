---
okf_version: "0.2"
type: Function
title: dispatch_remove_message
description: "Remove-from-project modal family handler (namespaced, ADR-0001 D3)."
resource: crates/oxide-app/src/app/dispatch/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/overlay/dispatch_remove_message
language: rust
---

# dispatch_remove_message

Remove-from-project modal family handler (namespaced, ADR-0001 D3).

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_remove_message(&mut self, msg: RemoveMsg) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Remove-from-project modal family handler (namespaced, ADR-0001 D3).

## Source
Lines 505–513 in `crates/oxide-app/src/app/dispatch/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/app/dispatch/overlay.md) |
