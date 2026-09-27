---
okf_version: "0.2"
type: Function
title: bus_ctx
resource: crates/oxide-erc/src/rules/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/tests/bus_ctx
language: rust
---

# bus_ctx

## Signature

```rust
fn bus_ctx(buses: Vec<crate::context::ErcBus>, labels: Vec<ErcLabel>) -> ErcContext
```

## Source
Lines 342–348 in `crates/oxide-erc/src/rules/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-erc/src/rules/tests.md) |
| calls | [ctx](/crates/oxide-erc/src/rules/tests/ctx.md) |
| called_by | [bus_bit_width_mismatch_accepts_matching_mid_bus_widths](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_accepts_matching_mid_bus_widths.md) |
| called_by | [bus_bit_width_mismatch_catches_mid_bus_range_labels](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_catches_mid_bus_range_labels.md) |
| called_by | [bus_bit_width_mismatch_ignores_a_label_off_every_bus](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_ignores_a_label_off_every_bus.md) |
| called_by | [bus_bit_width_mismatch_keeps_separate_buses_apart](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_keeps_separate_buses_apart.md) |
| called_by | [bus_bit_width_mismatch_still_catches_endpoint_range_labels](/crates/oxide-erc/src/rules/tests/bus_bit_width_mismatch_still_catches_endpoint_range_labels.md) |
