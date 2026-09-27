---
okf_version: "0.2"
type: Function
title: writable_and_capacity
description: "Mirror of `ensure_capacity`'s pure clamp math, so the arithmetic that"
resource: crates/oxide-gfx/src/pipeline/growth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/pipeline/growth/writable_and_capacity
language: rust
---

# writable_and_capacity

Mirror of `ensure_capacity`'s pure clamp math, so the arithmetic that

## Signature

```rust
fn writable_and_capacity(
        required: usize,
        current: usize,
        elem_size: usize,
        max_buffer_size: u64,
    ) -> (usize, usize)
```

## Docstring

Mirror of `ensure_capacity`'s pure clamp math, so the arithmetic that
decides how many elements fit under `max_buffer_size` is testable
without a GPU device.

## Source
Lines 91–105 in `crates/oxide-gfx/src/pipeline/growth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [growth](/crates/oxide-gfx/src/pipeline/growth.md) |
| called_by | [clamp_never_yields_a_power_of_two_past_the_limit](/crates/oxide-gfx/src/pipeline/growth/clamp_never_yields_a_power_of_two_past_the_limit.md) |
| called_by | [clamps_when_required_exceeds_the_device_limit](/crates/oxide-gfx/src/pipeline/growth/clamps_when_required_exceeds_the_device_limit.md) |
| called_by | [keeps_capacity_when_required_fits_current](/crates/oxide-gfx/src/pipeline/growth/keeps_capacity_when_required_fits_current.md) |
| called_by | [writes_everything_when_it_fits_and_grows_by_power_of_two](/crates/oxide-gfx/src/pipeline/growth/writes_everything_when_it_fits_and_grows_by_power_of_two.md) |
