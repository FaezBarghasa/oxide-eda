---
okf_version: "0.2"
type: Class
title: EmbeddedMqttBroker
description: Lightweight In-Process Embedded MQTT Broker for offline simulation.
resource: crates/oxide-proto/src/mqtt.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:32Z"
concept_id: crates/oxide-proto/src/mqtt/EmbeddedMqttBroker
language: rust
---

# EmbeddedMqttBroker

Lightweight In-Process Embedded MQTT Broker for offline simulation.

## Signature

```rust
pub struct EmbeddedMqttBroker
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Lightweight In-Process Embedded MQTT Broker for offline simulation.
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `subscriptions`
- `retained`
- `message_log`

## Source
Lines 32–39 in `crates/oxide-proto/src/mqtt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mqtt](/crates/oxide-proto/src/mqtt.md) |
