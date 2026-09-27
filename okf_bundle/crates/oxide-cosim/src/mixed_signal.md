---
okf_version: "0.2"
type: Module
title: mixed_signal
description: 12-State Mixed-Signal Event Synchronization Engine.
resource: crates/oxide-cosim/src/mixed_signal.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:17:02Z"
concept_id: crates/oxide-cosim/src/mixed_signal
language: rust
---

# mixed_signal

12-State Mixed-Signal Event Synchronization Engine.

## Docstring

12-State Mixed-Signal Event Synchronization Engine.

Conforms to Master Technical Directive §3.6:
- Full IEEE 1164 9-state logic + high-impedance decay and inertial charge states (12-State Model)
- AtoD / DtoA Gateways with Hermite interpolation and exponential ramps
- Lockstep dynamic timestep coupling between continuous MNA solver and discrete logic event queue

## Relationships

| Type | Target |
|------|--------|
| related | [Logic12State](/crates/oxide-cosim/src/mixed_signal/Logic12State.md) |
| related | [to_binary](/crates/oxide-cosim/src/mixed_signal/to_binary.md) |
| related | [resolve](/crates/oxide-cosim/src/mixed_signal/resolve.md) |
| related | [to_binary](/crates/oxide-cosim/src/mixed_signal/to_binary.md) |
| related | [resolve](/crates/oxide-cosim/src/mixed_signal/resolve.md) |
| related | [LogicEvent](/crates/oxide-cosim/src/mixed_signal/LogicEvent.md) |
| related | [eq](/crates/oxide-cosim/src/mixed_signal/eq.md) |
| related | [eq](/crates/oxide-cosim/src/mixed_signal/eq.md) |
| related | [partial_cmp](/crates/oxide-cosim/src/mixed_signal/partial_cmp.md) |
| related | [partial_cmp](/crates/oxide-cosim/src/mixed_signal/partial_cmp.md) |
| related | [cmp](/crates/oxide-cosim/src/mixed_signal/cmp.md) |
| related | [cmp](/crates/oxide-cosim/src/mixed_signal/cmp.md) |
| related | [LogicEventQueue](/crates/oxide-cosim/src/mixed_signal/LogicEventQueue.md) |
| related | [new](/crates/oxide-cosim/src/mixed_signal/new.md) |
| related | [push](/crates/oxide-cosim/src/mixed_signal/push.md) |
| related | [pop_before_or_at](/crates/oxide-cosim/src/mixed_signal/pop_before_or_at.md) |
| related | [next_event_time](/crates/oxide-cosim/src/mixed_signal/next_event_time.md) |
| related | [is_empty](/crates/oxide-cosim/src/mixed_signal/is_empty.md) |
| related | [clear](/crates/oxide-cosim/src/mixed_signal/clear.md) |
| related | [new](/crates/oxide-cosim/src/mixed_signal/new.md) |
| related | [push](/crates/oxide-cosim/src/mixed_signal/push.md) |
| related | [pop_before_or_at](/crates/oxide-cosim/src/mixed_signal/pop_before_or_at.md) |
| related | [next_event_time](/crates/oxide-cosim/src/mixed_signal/next_event_time.md) |
| related | [is_empty](/crates/oxide-cosim/src/mixed_signal/is_empty.md) |
| related | [clear](/crates/oxide-cosim/src/mixed_signal/clear.md) |
| related | [AtoDGateway](/crates/oxide-cosim/src/mixed_signal/AtoDGateway.md) |
| related | [new](/crates/oxide-cosim/src/mixed_signal/new.md) |
| related | [evaluate_trajectory](/crates/oxide-cosim/src/mixed_signal/evaluate_trajectory.md) |
| related | [new](/crates/oxide-cosim/src/mixed_signal/new.md) |
| related | [evaluate_trajectory](/crates/oxide-cosim/src/mixed_signal/evaluate_trajectory.md) |
| related | [DtoAGateway](/crates/oxide-cosim/src/mixed_signal/DtoAGateway.md) |
| related | [new](/crates/oxide-cosim/src/mixed_signal/new.md) |
| related | [update_state](/crates/oxide-cosim/src/mixed_signal/update_state.md) |
| related | [sample_voltage](/crates/oxide-cosim/src/mixed_signal/sample_voltage.md) |
| related | [new](/crates/oxide-cosim/src/mixed_signal/new.md) |
| related | [update_state](/crates/oxide-cosim/src/mixed_signal/update_state.md) |
| related | [sample_voltage](/crates/oxide-cosim/src/mixed_signal/sample_voltage.md) |
| related | [LockstepSynchronizer](/crates/oxide-cosim/src/mixed_signal/LockstepSynchronizer.md) |
| related | [new](/crates/oxide-cosim/src/mixed_signal/new.md) |
| related | [compute_synchronized_timestep](/crates/oxide-cosim/src/mixed_signal/compute_synchronized_timestep.md) |
| related | [advance_to](/crates/oxide-cosim/src/mixed_signal/advance_to.md) |
| related | [new](/crates/oxide-cosim/src/mixed_signal/new.md) |
| related | [compute_synchronized_timestep](/crates/oxide-cosim/src/mixed_signal/compute_synchronized_timestep.md) |
| related | [advance_to](/crates/oxide-cosim/src/mixed_signal/advance_to.md) |
| related | [test_logic_12_state_resolution](/crates/oxide-cosim/src/mixed_signal/test_logic_12_state_resolution.md) |
| related | [test_atod_gateway_threshold_interpolation](/crates/oxide-cosim/src/mixed_signal/test_atod_gateway_threshold_interpolation.md) |
| related | [test_dtoa_gateway_exponential_ramp](/crates/oxide-cosim/src/mixed_signal/test_dtoa_gateway_exponential_ramp.md) |
| related | [test_lockstep_timestep_truncation](/crates/oxide-cosim/src/mixed_signal/test_lockstep_timestep_truncation.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
