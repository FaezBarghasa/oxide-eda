---
okf_version: "0.2"
type: Function
title: dispatch_parameter_manager_message
description: "Parameter Manager dialog handler (namespaced family, ADR-0001 D3)."
resource: crates/oxide-app/src/app/dispatch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/mod/dispatch_parameter_manager_message
language: rust
---

# dispatch_parameter_manager_message

Parameter Manager dialog handler (namespaced family, ADR-0001 D3).

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_parameter_manager_message(
        &mut self,
        msg: ParameterManagerMsg,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Parameter Manager dialog handler (namespaced family, ADR-0001 D3).

## Source
Lines 625–644 in `crates/oxide-app/src/app/dispatch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dispatch](/crates/oxide-app/src/app/dispatch/mod.md) |
