---
okf_version: "0.2"
type: Function
title: records_naming
description: Records currently in the ring that name this command id.
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/records_naming
language: rust
---

# records_naming

Records currently in the ring that name this command id.

## Signature

```rust
fn records_naming(id: &str) -> usize
```

## Docstring

Records currently in the ring that name this command id.

Counting *total* entries instead would make these tests depend on
whatever else the binary happened to log in a neighbouring thread —
which is exactly how the first version of
`asking_whether_a_command_resolves_reports_nothing` failed in CI
(`left: 2, right: 1`). The claim under test is about records naming
this id, so count those.

## Source
Lines 488–494 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| calls | [recent_entries](/crates/oxide-app/src/diagnostics/recent_entries.md) |
| called_by | [asking_whether_a_command_resolves_reports_nothing](/crates/oxide-app/src/app/command/bridge/asking_whether_a_command_resolves_reports_nothing.md) |
| called_by | [dispatching_an_unmapped_command_still_reports_it](/crates/oxide-app/src/app/command/bridge/dispatching_an_unmapped_command_still_reports_it.md) |
