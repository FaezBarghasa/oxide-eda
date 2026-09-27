---
okf_version: "0.2"
type: Function
title: evaluate
resource: crates/oxide-widgets/src/passive_calculator/network.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/network/evaluate
language: rust
---

# evaluate

## Signature

```rust
impl Network { fn evaluate(&self, kind: ComponentKind, bound: Bound) -> f64 }
```

## Source
Lines 96–124 in `crates/oxide-widgets/src/passive_calculator/network.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network](/crates/oxide-widgets/src/passive_calculator/network.md) |
| calls | [is_additive](/crates/oxide-widgets/src/passive_calculator/network/is_additive.md) |
| calls | [parallel_value](/crates/oxide-widgets/src/passive_calculator/network/parallel_value.md) |
