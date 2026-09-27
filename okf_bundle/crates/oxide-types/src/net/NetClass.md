---
okf_version: "0.2"
type: Class
title: NetClass
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/oxide-types/src/net.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:59:29Z"
concept_id: crates/oxide-types/src/net/NetClass
language: rust
---

# NetClass

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct NetClass
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `clearance`
- `trace_width`
- `via_diameter`
- `via_drill`
- `diff_pair_gap`
- `diff_pair_width`

## Source
Lines 19–33 in `crates/oxide-types/src/net.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [net](/crates/oxide-types/src/net.md) |
| called_by | [deserialize](/crates/oxide-rules/src/scope/deserialize.md) |
| called_by | [phase_skew_violation](/crates/oxide-rules/src/violation/phase_skew_violation.md) |
| called_by | [return_path_split_crossing](/crates/oxide-rules/src/violation/return_path_split_crossing.md) |
| called_by | [test_clearance_matrix_evaluation](/crates/oxide-rules/tests/rules_tests/test_clearance_matrix_evaluation.md) |
| called_by | [test_component_clearance_and_routing_layer_and_phase_skew_validation](/crates/oxide-rules/tests/rules_tests/test_component_clearance_and_routing_layer_and_phase_skew_validation.md) |
| called_by | [test_hierarchical_net_class_override](/crates/oxide-rules/tests/rules_tests/test_hierarchical_net_class_override.md) |
| called_by | [test_hierarchical_net_specific_override](/crates/oxide-rules/tests/rules_tests/test_hierarchical_net_specific_override.md) |
