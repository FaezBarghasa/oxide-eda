---
okf_version: "0.2"
type: Class
title: PrintPreviewMsg
description: Print-preview modal message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/messages.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/contracts/messages/PrintPreviewMsg
language: rust
---

# PrintPreviewMsg

Print-preview modal message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum PrintPreviewMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Print-preview modal message family (ADR-0001 D3). Namespaced under
`Message::PrintPreview` and routed to `dispatch_print_preview_message`.
[derive(Debug, Clone)]

## Source
Lines 326–394 in `crates/oxide-app/src/app/contracts/messages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/app/contracts/messages.md) |
