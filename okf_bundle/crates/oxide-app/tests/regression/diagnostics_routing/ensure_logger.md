---
okf_version: "0.2"
type: Function
title: ensure_logger
description: "Install the process logger so `tracing::*` records actually reach the"
resource: crates/oxide-app/tests/regression/diagnostics_routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/diagnostics_routing/ensure_logger
language: rust
---

# ensure_logger

Install the process logger so `tracing::*` records actually reach the

## Signature

```rust
fn ensure_logger()
```

## Docstring

Install the process logger so `tracing::*` records actually reach the
ring buffer. `log::set_boxed_logger` accepts exactly one call per
process, so a second one from another test in this binary is the
already-installed error and is nothing to act on.

## Source
Lines 26–28 in `crates/oxide-app/tests/regression/diagnostics_routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics_routing](/crates/oxide-app/tests/regression/diagnostics_routing.md) |
| calls | [init_logging](/crates/oxide-app/src/diagnostics/init_logging.md) |
| called_by | [a_record_emitted_during_a_preferences_message_reaches_the_panel_snapshot](/crates/oxide-app/tests/regression/diagnostics_routing/a_record_emitted_during_a_preferences_message_reaches_the_panel_snapshot.md) |
| called_by | [the_panel_snapshot_is_refreshed_on_every_preferences_message](/crates/oxide-app/tests/regression/diagnostics_routing/the_panel_snapshot_is_refreshed_on_every_preferences_message.md) |
