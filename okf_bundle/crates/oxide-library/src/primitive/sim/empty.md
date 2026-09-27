---
okf_version: "0.2"
type: Function
title: empty
resource: crates/oxide-library/src/primitive/sim.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:11:54Z"
concept_id: crates/oxide-library/src/primitive/sim/empty
language: rust
---

# empty

## Signature

```rust
impl SimModel { pub fn empty(name: impl Into<String>, kind: SimKind) -> Self }
```

## Visibility

- `pub`

## Source
Lines 61–74 in `crates/oxide-library/src/primitive/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-library/src/primitive/sim.md) |
| calls | [default_sim_version](/crates/oxide-library/src/primitive/sim/default_sim_version.md) |
