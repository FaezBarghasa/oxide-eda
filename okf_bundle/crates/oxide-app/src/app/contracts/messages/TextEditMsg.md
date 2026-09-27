---
okf_version: "0.2"
type: Class
title: TextEditMsg
description: In-canvas text-edit message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/messages.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/contracts/messages/TextEditMsg
language: rust
---

# TextEditMsg

In-canvas text-edit message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum TextEditMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

In-canvas text-edit message family (ADR-0001 D3). Namespaced under
`Message::TextEdit` and routed to `dispatch_text_edit_message`.
[derive(Debug, Clone)]

## Source
Lines 132–137 in `crates/oxide-app/src/app/contracts/messages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/app/contracts/messages.md) |
