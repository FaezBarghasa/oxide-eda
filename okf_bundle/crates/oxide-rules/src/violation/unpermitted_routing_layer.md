---
okf_version: "0.2"
type: Function
title: unpermitted_routing_layer
resource: crates/oxide-rules/src/violation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:27:50Z"
concept_id: crates/oxide-rules/src/violation/unpermitted_routing_layer
language: rust
---

# unpermitted_routing_layer

## Signature

```rust
impl RuleViolation { pub fn unpermitted_routing_layer(
        net: &str,
        scope: RuleScope,
        layer: &str,
        permitted_layers: &[String],
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 191–209 in `crates/oxide-rules/src/violation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [violation](/crates/oxide-rules/src/violation.md) |
