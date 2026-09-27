---
okf_version: "0.2"
type: Function
title: entries
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/entries
language: rust
---

# entries

## Signature

```rust
fn entries() -> &'static Mutex<VecDeque<DiagnosticEntry>>
```

## Source
Lines 133–135 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| called_by | [push_entry](/crates/oxide-app/src/diagnostics/push_entry.md) |
| called_by | [recent_entries](/crates/oxide-app/src/diagnostics/recent_entries.md) |
