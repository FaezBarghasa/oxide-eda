---
okf_version: "0.2"
type: Function
title: unsubscribe
description: Client unsubscribes from a topic pattern.
resource: crates/oxide-proto/src/mqtt.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:32Z"
concept_id: crates/oxide-proto/src/mqtt/unsubscribe
language: rust
---

# unsubscribe

Client unsubscribes from a topic pattern.

## Signature

```rust
impl EmbeddedMqttBroker { pub fn unsubscribe(&mut self, client_id: &str, topic_filter: &str) }
```

## Visibility

- `pub`

## Docstring

Client unsubscribes from a topic pattern.

## Source
Lines 54–58 in `crates/oxide-proto/src/mqtt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mqtt](/crates/oxide-proto/src/mqtt.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
