---
okf_version: "0.2"
type: Function
title: format_value
resource: crates/oxide-widgets/src/passive_calculator/network.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/network/format_value
language: rust
---

# format_value

## Signature

```rust
pub fn format_value(value: f64, kind: ComponentKind) -> String
```

## Visibility

- `pub`

## Source
Lines 243–253 in `crates/oxide-widgets/src/passive_calculator/network.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network](/crates/oxide-widgets/src/passive_calculator/network.md) |
| calls | [best_prefix](/crates/oxide-widgets/src/passive_calculator/network/best_prefix.md) |
| called_by | [format_target](/crates/oxide-widgets/src/passive_calculator/control/format_target.md) |
