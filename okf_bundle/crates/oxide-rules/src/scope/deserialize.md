---
okf_version: "0.2"
type: Function
title: deserialize
resource: crates/oxide-rules/src/scope.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:31:50Z"
concept_id: crates/oxide-rules/src/scope/deserialize
language: rust
---

# deserialize

## Signature

```rust
impl RuleScope { fn deserialize(deserializer: D) -> Result<Self, D::Error> }
```

## Type Parameters

- `D`

## Source
Lines 63–91 in `crates/oxide-rules/src/scope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scope](/crates/oxide-rules/src/scope.md) |
| calls | [Room](/crates/oxide-engine/src/room/Room.md) |
| calls | [NetClass](/crates/oxide-types/src/net/NetClass.md) |
| calls | [Net](/crates/oxide-types/src/net/Net.md) |
