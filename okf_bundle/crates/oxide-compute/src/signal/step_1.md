---
okf_version: "0.2"
type: Function
title: step
resource: crates/oxide-compute/src/signal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:22Z"
concept_id: crates/oxide-compute/src/signal/step_1
language: rust
---

# step

## Signature

```rust
pub fn step(
        &mut self,
        fields: &mut [FieldCell],
        steps: u32,
    ) -> Result<(), crate::backend::ComputeError>
```

## Visibility

- `pub`

## Source
Lines 74–123 in `crates/oxide-compute/src/signal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [signal](/crates/oxide-compute/src/signal.md) |
