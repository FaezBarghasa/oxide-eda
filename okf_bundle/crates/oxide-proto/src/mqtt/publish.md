---
okf_version: "0.2"
type: Function
title: publish
description: Publishes a message through the broker.
resource: crates/oxide-proto/src/mqtt.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:32Z"
concept_id: crates/oxide-proto/src/mqtt/publish
language: rust
---

# publish

Publishes a message through the broker.

## Signature

```rust
impl EmbeddedMqttBroker { pub fn publish(&mut self, message: MqttMessage) -> Vec<String> }
```

## Visibility

- `pub`

## Docstring

Publishes a message through the broker.

## Source
Lines 61–78 in `crates/oxide-proto/src/mqtt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mqtt](/crates/oxide-proto/src/mqtt.md) |
| calls | [topic_matches](/crates/oxide-proto/src/mqtt/topic_matches.md) |
