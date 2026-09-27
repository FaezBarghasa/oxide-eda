---
okf_version: "0.2"
type: Function
title: dispatch_rename_message
description: "Rename modal family handler (namespaced, ADR-0001 D3)."
resource: crates/oxide-app/src/app/dispatch/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/overlay/dispatch_rename_message_1
language: rust
---

# dispatch_rename_message

Rename modal family handler (namespaced, ADR-0001 D3).

## Signature

```rust
pub(crate) fn dispatch_rename_message(&mut self, msg: RenameMsg) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Rename modal family handler (namespaced, ADR-0001 D3).

## Source
Lines 487–502 in `crates/oxide-app/src/app/dispatch/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/app/dispatch/overlay.md) |
