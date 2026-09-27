---
okf_version: "0.2"
type: Function
title: topic_matches
description: "Helper evaluating MQTT wildcard topic matching (`+` single level, `#` multi-level)."
resource: crates/oxide-proto/src/mqtt.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:32Z"
concept_id: crates/oxide-proto/src/mqtt/topic_matches
language: rust
---

# topic_matches

Helper evaluating MQTT wildcard topic matching (`+` single level, `#` multi-level).

## Signature

```rust
fn topic_matches(filter: &str, topic: &str) -> bool
```

## Docstring

Helper evaluating MQTT wildcard topic matching (`+` single level, `#` multi-level).

## Source
Lines 87–114 in `crates/oxide-proto/src/mqtt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mqtt](/crates/oxide-proto/src/mqtt.md) |
| called_by | [publish](/crates/oxide-proto/src/mqtt/publish.md) |
