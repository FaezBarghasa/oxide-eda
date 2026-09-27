---
okf_version: "0.2"
type: Function
title: validate_routing_layer
description: Validate that a routed trace is placed on an authorized copper layer.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/validate_routing_layer_1
language: rust
---

# validate_routing_layer

Validate that a routed trace is placed on an authorized copper layer.

## Signature

```rust
pub fn validate_routing_layer(
        &self,
        net: &str,
        net_class: Option<&str>,
        room: Option<&str>,
        layer: &str,
    ) -> Result<(), RuleViolation>
```

## Decorators

- `allow(clippy::result_large_err)`

## Visibility

- `pub`

## Docstring

Validate that a routed trace is placed on an authorized copper layer.
[allow(clippy::result_large_err)]

## Source
Lines 488–506 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
