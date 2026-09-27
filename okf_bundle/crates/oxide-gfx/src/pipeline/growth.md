---
okf_version: "0.2"
type: Module
title: growth
description: Shared instance/vertex buffer growth for the GPU pipelines.
resource: crates/oxide-gfx/src/pipeline/growth.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/pipeline/growth
language: rust
---

# growth

Shared instance/vertex buffer growth for the GPU pipelines.

## Docstring

Shared instance/vertex buffer growth for the GPU pipelines.

CLEAN ROOM DECLARATION
This module was written without reference to GPL-licensed software.
Sources: wgpu/WGSL public docs.

## Relationships

| Type | Target |
|------|--------|
| related | [GrowthParams](/crates/oxide-gfx/src/pipeline/growth/GrowthParams.md) |
| related | [ensure_capacity](/crates/oxide-gfx/src/pipeline/growth/ensure_capacity.md) |
| related | [warn_clamp_once](/crates/oxide-gfx/src/pipeline/growth/warn_clamp_once.md) |
| related | [writable_and_capacity](/crates/oxide-gfx/src/pipeline/growth/writable_and_capacity.md) |
| related | [writes_everything_when_it_fits_and_grows_by_power_of_two](/crates/oxide-gfx/src/pipeline/growth/writes_everything_when_it_fits_and_grows_by_power_of_two.md) |
| related | [keeps_capacity_when_required_fits_current](/crates/oxide-gfx/src/pipeline/growth/keeps_capacity_when_required_fits_current.md) |
| related | [clamps_when_required_exceeds_the_device_limit](/crates/oxide-gfx/src/pipeline/growth/clamps_when_required_exceeds_the_device_limit.md) |
| related | [clamp_never_yields_a_power_of_two_past_the_limit](/crates/oxide-gfx/src/pipeline/growth/clamp_never_yields_a_power_of_two_past_the_limit.md) |
