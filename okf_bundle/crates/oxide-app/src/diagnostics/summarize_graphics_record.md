---
okf_version: "0.2"
type: Function
title: summarize_graphics_record
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/summarize_graphics_record
language: rust
---

# summarize_graphics_record

## Signature

```rust
fn summarize_graphics_record(rendered: &str) -> Option<(String, String)>
```

## Source
Lines 202–234 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| calls | [extract_named_field](/crates/oxide-app/src/diagnostics/extract_named_field.md) |
| calls | [extract_bracket_items](/crates/oxide-app/src/diagnostics/extract_bracket_items.md) |
| calls | [join_preview](/crates/oxide-app/src/diagnostics/join_preview.md) |
| called_by | [summarize_record](/crates/oxide-app/src/diagnostics/summarize_record.md) |
