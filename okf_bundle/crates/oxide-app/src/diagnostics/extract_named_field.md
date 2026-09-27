---
okf_version: "0.2"
type: Function
title: extract_named_field
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/extract_named_field
language: rust
---

# extract_named_field

## Signature

```rust
fn extract_named_field(rendered: &str, field_name: &str) -> Option<String>
```

## Source
Lines 262–272 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [summarize_graphics_record](/crates/oxide-app/src/diagnostics/summarize_graphics_record.md) |
