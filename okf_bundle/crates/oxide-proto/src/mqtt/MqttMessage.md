---
okf_version: "0.2"
type: Class
title: MqttMessage
description: Recorded MQTT Message.
resource: crates/oxide-proto/src/mqtt.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:32Z"
concept_id: crates/oxide-proto/src/mqtt/MqttMessage
language: rust
---

# MqttMessage

Recorded MQTT Message.

## Signature

```rust
pub struct MqttMessage
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Recorded MQTT Message.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `topic`
- `payload`
- `qos`
- `retain`
- `timestamp_ms`

## Source
Lines 16–22 in `crates/oxide-proto/src/mqtt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mqtt](/crates/oxide-proto/src/mqtt.md) |
