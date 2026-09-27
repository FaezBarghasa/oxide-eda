---
okf_version: "0.2"
type: Class
title: BomPreviewMsg
description: BOM-preview modal message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/messages.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/contracts/messages/BomPreviewMsg
language: rust
---

# BomPreviewMsg

BOM-preview modal message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum BomPreviewMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

BOM-preview modal message family (ADR-0001 D3). Namespaced under
`Message::BomPreview` and routed to `dispatch_bom_preview_message`.
[derive(Debug, Clone)]

## Source
Lines 271–321 in `crates/oxide-app/src/app/contracts/messages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/app/contracts/messages.md) |
