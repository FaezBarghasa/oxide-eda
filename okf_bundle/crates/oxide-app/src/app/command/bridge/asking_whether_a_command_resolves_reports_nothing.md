---
okf_version: "0.2"
type: Function
title: asking_whether_a_command_resolves_reports_nothing
description: "#619 — a query must not narrate. The palette filters its rows by"
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/asking_whether_a_command_resolves_reports_nothing
language: rust
---

# asking_whether_a_command_resolves_reports_nothing

#619 — a query must not narrate. The palette filters its rows by

## Signature

```rust
fn asking_whether_a_command_resolves_reports_nothing()
```

## Decorators

- `test`

## Docstring

#619 — a query must not narrate. The palette filters its rows by
asking whether each catalog id resolves; routing that through the
dispatch entry point put one warning per unmapped id into the
Messages panel on every rebuild — 2432 records in one observed
session, against a 200-entry ring buffer, so every real
diagnostic was evicted before it could be read.
[test]

## Source
Lines 503–528 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| calls | [serial](/crates/oxide-app/src/app/command/bridge/serial.md) |
| calls | [init_logging](/crates/oxide-app/src/diagnostics/init_logging.md) |
| calls | [records_naming](/crates/oxide-app/src/app/command/bridge/records_naming.md) |
