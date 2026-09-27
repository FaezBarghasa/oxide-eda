---
okf_version: "0.2"
type: Function
title: serialize
resource: crates/oxide-rules/src/scope.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:31:50Z"
concept_id: crates/oxide-rules/src/scope/serialize
language: rust
---

# serialize

## Signature

```rust
impl RuleScope { fn serialize(&self, serializer: S) -> Result<S::Ok, S::Error> }
```

## Type Parameters

- `S`

## Source
Lines 48–59 in `crates/oxide-rules/src/scope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scope](/crates/oxide-rules/src/scope.md) |
