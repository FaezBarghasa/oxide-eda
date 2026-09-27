---
okf_version: "0.2"
type: Class
title: LockstepSynchronizer
description: Lockstep Synchronizer coupling the continuous MNA solver with the discrete event queue.
resource: crates/oxide-cosim/src/mixed_signal.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:17:02Z"
concept_id: crates/oxide-cosim/src/mixed_signal/LockstepSynchronizer
language: rust
---

# LockstepSynchronizer

Lockstep Synchronizer coupling the continuous MNA solver with the discrete event queue.

## Signature

```rust
pub struct LockstepSynchronizer
```

## Decorators

- `derive(Debug, Default)`

## Visibility

- `pub`

## Docstring

Lockstep Synchronizer coupling the continuous MNA solver with the discrete event queue.
[derive(Debug, Default)]

## Methods

- `current_time_s`
- `event_queue`
- `atod_gateways`
- `dtoa_gateways`

## Source
Lines 281–286 in `crates/oxide-cosim/src/mixed_signal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mixed_signal](/crates/oxide-cosim/src/mixed_signal.md) |
