---
okf_version: "0.2"
type: Function
title: a_record_emitted_during_a_preferences_message_reaches_the_panel_snapshot
description: "The record is emitted at `Error`, which every level except `off`"
resource: crates/oxide-app/tests/regression/diagnostics_routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/diagnostics_routing/a_record_emitted_during_a_preferences_message_reaches_the_panel_snapshot
language: rust
---

# a_record_emitted_during_a_preferences_message_reaches_the_panel_snapshot

The record is emitted at `Error`, which every level except `off`

## Signature

```rust
fn a_record_emitted_during_a_preferences_message_reaches_the_panel_snapshot()
```

## Decorators

- `test`

## Docstring

The record is emitted at `Error`, which every level except `off`
passes, so this does not depend on the developer's `RUST_LOG`.
[test]

## Source
Lines 33–63 in `crates/oxide-app/tests/regression/diagnostics_routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics_routing](/crates/oxide-app/tests/regression/diagnostics_routing.md) |
| calls | [ensure_logger](/crates/oxide-app/tests/regression/diagnostics_routing/ensure_logger.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
