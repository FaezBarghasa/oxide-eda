---
okf_version: "0.2"
type: Function
title: the_panel_snapshot_is_refreshed_on_every_preferences_message
description: "The snapshot has to be the *current* buffer, not a stale copy taken"
resource: crates/oxide-app/tests/regression/diagnostics_routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/diagnostics_routing/the_panel_snapshot_is_refreshed_on_every_preferences_message
language: rust
---

# the_panel_snapshot_is_refreshed_on_every_preferences_message

The snapshot has to be the *current* buffer, not a stale copy taken

## Signature

```rust
fn the_panel_snapshot_is_refreshed_on_every_preferences_message()
```

## Decorators

- `test`

## Docstring

The snapshot has to be the *current* buffer, not a stale copy taken
once at boot. Pins that the republish happens per message.
[test]

## Source
Lines 68–96 in `crates/oxide-app/tests/regression/diagnostics_routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics_routing](/crates/oxide-app/tests/regression/diagnostics_routing.md) |
| calls | [ensure_logger](/crates/oxide-app/tests/regression/diagnostics_routing/ensure_logger.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
