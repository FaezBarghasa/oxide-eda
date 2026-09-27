---
okf_version: "0.2"
type: Class
title: LogicEvent
description: Discrete Mixed-Signal Event in the event queue.
resource: crates/oxide-cosim/src/mixed_signal.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:17:02Z"
concept_id: crates/oxide-cosim/src/mixed_signal/LogicEvent
language: rust
---

# LogicEvent

Discrete Mixed-Signal Event in the event queue.

## Signature

```rust
pub struct LogicEvent
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Discrete Mixed-Signal Event in the event queue.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `timestamp_s`
- `signal_id`
- `new_state`

## Source
Lines 75–79 in `crates/oxide-cosim/src/mixed_signal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mixed_signal](/crates/oxide-cosim/src/mixed_signal.md) |
