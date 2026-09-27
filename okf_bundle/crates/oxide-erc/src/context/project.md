---
okf_version: "0.2"
type: Function
title: project
resource: crates/oxide-erc/src/context.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/context/project
language: rust
---

# project

## Signature

```rust
impl ErcContext { fn project(snapshot: &SchematicSheet, children: HashMap<String, ErcContext>) -> Self }
```

## Source
Lines 222–360 in `crates/oxide-erc/src/context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context](/crates/oxide-erc/src/context.md) |
| calls | [point_is_connected](/crates/oxide-erc/src/context/point_is_connected.md) |
| calls | [summarize_nets](/crates/oxide-erc/src/context/summarize_nets.md) |
