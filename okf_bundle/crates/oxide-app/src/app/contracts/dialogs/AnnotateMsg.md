---
okf_version: "0.2"
type: Class
title: AnnotateMsg
description: Annotate dialog message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/AnnotateMsg
language: rust
---

# AnnotateMsg

Annotate dialog message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum AnnotateMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Annotate dialog message family (ADR-0001 D3). Namespaced under
`Message::Annotate` and routed to `dispatch_annotate_message`.
[derive(Debug, Clone)]

## Source
Lines 50–68 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
