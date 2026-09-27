---
okf_version: "0.2"
type: Class
title: Net
description: "A logical net: a set of electrically-connected terminals derived from the"
resource: crates/oxide-types/src/net.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:59:29Z"
concept_id: crates/oxide-types/src/net/Net
language: rust
---

# Net

A logical net: a set of electrically-connected terminals derived from the

## Signature

```rust
pub struct Net
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A logical net: a set of electrically-connected terminals derived from the
schematic (wires + junctions + labels + pins). `id` is a build-time stable
number (also usable as the PCB net number); `name` comes from the
highest-priority label on the net, or is auto-assigned when unlabelled.

`wires` and `junctions` are the schematic elements the net occupies — the
membership the net-flood highlights and the ratsnest reads. `class` is a
*project-rules* concern layered on top of connectivity: the connectivity
builder leaves it `None`, and a later pass assigns a [`NetClassId`].
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `id`
- `name`
- `class`
- `wires`
- `junctions`
- `terminals`

## Source
Lines 142–152 in `crates/oxide-types/src/net.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [net](/crates/oxide-types/src/net.md) |
| called_by | [evaluate_rule](/crates/oxide-erc-dsl/src/compiler/evaluate_rule.md) |
| called_by | [subject_ref](/crates/oxide-erc-dsl/src/compiler/subject_ref.md) |
| called_by | [add_net](/crates/oxide-net/src/harness/add_net.md) |
| called_by | [deserialize](/crates/oxide-rules/src/scope/deserialize.md) |
| called_by | [test_hierarchical_net_specific_override](/crates/oxide-rules/tests/rules_tests/test_hierarchical_net_specific_override.md) |
