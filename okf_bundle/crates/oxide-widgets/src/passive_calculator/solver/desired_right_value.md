---
okf_version: "0.2"
type: Function
title: desired_right_value
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/desired_right_value
language: rust
---

# desired_right_value

## Signature

```rust
fn desired_right_value(kind: ComponentKind, connection: Connection, left: f64, target: f64) -> f64
```

## Source
Lines 459–470 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [is_additive](/crates/oxide-widgets/src/passive_calculator/network/is_additive.md) |
| called_by | [find_balanced_level_four_brackets](/crates/oxide-widgets/src/passive_calculator/solver/find_balanced_level_four_brackets.md) |
| called_by | [find_level_three_brackets](/crates/oxide-widgets/src/passive_calculator/solver/find_level_three_brackets.md) |
| called_by | [find_unbalanced_level_four_brackets](/crates/oxide-widgets/src/passive_calculator/solver/find_unbalanced_level_four_brackets.md) |
| called_by | [generate_target_brackets](/crates/oxide-widgets/src/passive_calculator/solver/generate_target_brackets.md) |
