---
okf_version: "0.2"
type: Class
title: WindowMsg
description: "Window lifecycle, docking, and native-chrome message family"
resource: crates/oxide-app/src/app/contracts/messages.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/contracts/messages/WindowMsg
language: rust
---

# WindowMsg

Window lifecycle, docking, and native-chrome message family

## Signature

```rust
pub enum WindowMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Window lifecycle, docking, and native-chrome message family
(ADR-0001 D3). Namespaced under `Message::Window`, routed to
`dispatch_window_message`.
[derive(Debug, Clone)]

## Methods

- `modal`
- `id`
- `path`
- `id`
- `kind`
- `id`
- `modal`
- `direction`

## Source
Lines 9–98 in `crates/oxide-app/src/app/contracts/messages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/app/contracts/messages.md) |
