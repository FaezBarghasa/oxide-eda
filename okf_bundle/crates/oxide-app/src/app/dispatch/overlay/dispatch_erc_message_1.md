---
okf_version: "0.2"
type: Function
title: dispatch_erc_message
description: "ERC dialog family handler (namespaced, ADR-0001 D3)."
resource: crates/oxide-app/src/app/dispatch/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/overlay/dispatch_erc_message_1
language: rust
---

# dispatch_erc_message

ERC dialog family handler (namespaced, ADR-0001 D3).

## Signature

```rust
pub(crate) fn dispatch_erc_message(&mut self, msg: ErcMsg) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

ERC dialog family handler (namespaced, ADR-0001 D3).

## Source
Lines 337–434 in `crates/oxide-app/src/app/dispatch/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/app/dispatch/overlay.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [write_pin_matrix_overrides](/crates/oxide-app/src/fonts/erc/write_pin_matrix_overrides.md) |
