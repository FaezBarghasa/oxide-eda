---
okf_version: "0.2"
type: Function
title: serial
description: "The diagnostics ring is one process-wide `Mutex<VecDeque<_>>`"
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/serial
language: rust
---

# serial

The diagnostics ring is one process-wide `Mutex<VecDeque<_>>`

## Signature

```rust
fn serial() -> std::sync::MutexGuard<'static, ()>
```

## Docstring

The diagnostics ring is one process-wide `Mutex<VecDeque<_>>`
(`diagnostics.rs`), and cargo runs the `oxide-app --lib` tests in
parallel threads. The two reporting tests below both read it around
an action, so they have to take turns — otherwise one sees the
other's record land between its `before` and its assert.

Poison is recovered rather than propagated: a panic in one of these
two is a real failure to report, not a reason to fail the other one
for an unrelated reason.

## Source
Lines 475–478 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| called_by | [asking_whether_a_command_resolves_reports_nothing](/crates/oxide-app/src/app/command/bridge/asking_whether_a_command_resolves_reports_nothing.md) |
| called_by | [dispatching_an_unmapped_command_still_reports_it](/crates/oxide-app/src/app/command/bridge/dispatching_an_unmapped_command_still_reports_it.md) |
