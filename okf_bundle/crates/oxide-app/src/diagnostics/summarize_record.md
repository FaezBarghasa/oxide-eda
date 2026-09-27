---
okf_version: "0.2"
type: Function
title: summarize_record
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/summarize_record
language: rust
---

# summarize_record

## Signature

```rust
fn summarize_record(target: &str, rendered: &str) -> (String, String)
```

## Source
Lines 192–200 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| calls | [summarize_graphics_record](/crates/oxide-app/src/diagnostics/summarize_graphics_record.md) |
| calls | [diagnostic_code_from_target](/crates/oxide-app/src/diagnostics/diagnostic_code_from_target.md) |
| calls | [compact_message](/crates/oxide-app/src/diagnostics/compact_message.md) |
| called_by | [log](/crates/oxide-app/src/diagnostics/log.md) |
