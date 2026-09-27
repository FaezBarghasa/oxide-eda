---
okf_version: "0.2"
type: Function
title: latch
description: "Recover a poisoned latch instead of panicking on it: the set is a"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/latch
language: rust
---

# latch

Recover a poisoned latch instead of panicking on it: the set is a

## Signature

```rust
fn latch() -> std::sync::MutexGuard<'static, HashSet<PathBuf>>
```

## Docstring

Recover a poisoned latch instead of panicking on it: the set is a
plain collection of paths that no unwinding writer can leave
half-updated, and losing the UI thread over a de-duplication cache
would be a worse failure than the one being reported.

## Source
Lines 131–135 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [reported_failures](/crates/oxide-app/src/fonts/prefs_file/reported_failures.md) |
| called_by | [forget_refusal](/crates/oxide-app/src/fonts/prefs_file/forget_refusal.md) |
| called_by | [report_refusal](/crates/oxide-app/src/fonts/prefs_file/report_refusal.md) |
