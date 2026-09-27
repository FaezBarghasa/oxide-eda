---
okf_version: "0.2"
type: Function
title: dispatching_an_unmapped_command_still_reports_it
description: "The other half: dispatching an unmapped command still reports, so"
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/dispatching_an_unmapped_command_still_reports_it
language: rust
---

# dispatching_an_unmapped_command_still_reports_it

The other half: dispatching an unmapped command still reports, so

## Signature

```rust
fn dispatching_an_unmapped_command_still_reports_it()
```

## Decorators

- `test`

## Docstring

The other half: dispatching an unmapped command still reports, so
the split silenced the query and not the failure.
[test]

## Source
Lines 533–553 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| calls | [serial](/crates/oxide-app/src/app/command/bridge/serial.md) |
| calls | [init_logging](/crates/oxide-app/src/diagnostics/init_logging.md) |
| calls | [records_naming](/crates/oxide-app/src/app/command/bridge/records_naming.md) |
