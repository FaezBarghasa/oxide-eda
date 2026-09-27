---
okf_version: "0.2"
type: Function
title: sim_set_enabled_round_trip_clears_state
description: Enable → disable round-trip clears every Sim-side field.
resource: crates/oxide-app/src/library/editor/sim/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:12:05Z"
concept_id: crates/oxide-app/src/library/editor/sim/mod/sim_set_enabled_round_trip_clears_state
language: rust
---

# sim_set_enabled_round_trip_clears_state

Enable → disable round-trip clears every Sim-side field.

## Signature

```rust
fn sim_set_enabled_round_trip_clears_state()
```

## Decorators

- `test`

## Docstring

Enable → disable round-trip clears every Sim-side field.
[test]

## Source
Lines 373–397 in `crates/oxide-app/src/library/editor/sim/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/editor/sim/mod.md) |
| calls | [fixture_editor](/crates/oxide-app/src/library/editor/sim/mod/fixture_editor.md) |
| calls | [apply](/crates/oxide-app/src/library/editor/sim/mod/apply.md) |
