---
okf_version: "0.2"
type: Function
title: log
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/log
language: rust
---

# log

## Signature

```rust
impl OxideLogger { fn log(&self, record: &Record<'_>) }
```

## Source
Lines 109–124 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| calls | [metadata](/crates/oxide-output/src/substitution/metadata.md) |
| calls | [summarize_record](/crates/oxide-app/src/diagnostics/summarize_record.md) |
| calls | [push_entry](/crates/oxide-app/src/diagnostics/push_entry.md) |
