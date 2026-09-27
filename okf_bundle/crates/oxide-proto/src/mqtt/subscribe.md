---
okf_version: "0.2"
type: Function
title: subscribe
description: Client subscribes to a topic pattern.
resource: crates/oxide-proto/src/mqtt.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:32Z"
concept_id: crates/oxide-proto/src/mqtt/subscribe
language: rust
---

# subscribe

Client subscribes to a topic pattern.

## Signature

```rust
impl EmbeddedMqttBroker { pub fn subscribe(&mut self, client_id: impl Into<String>, topic_filter: impl Into<String>) }
```

## Visibility

- `pub`

## Docstring

Client subscribes to a topic pattern.

## Source
Lines 47–51 in `crates/oxide-proto/src/mqtt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mqtt](/crates/oxide-proto/src/mqtt.md) |
