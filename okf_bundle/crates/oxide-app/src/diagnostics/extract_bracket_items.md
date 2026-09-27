---
okf_version: "0.2"
type: Function
title: extract_bracket_items
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/extract_bracket_items
language: rust
---

# extract_bracket_items

## Signature

```rust
fn extract_bracket_items(rendered: &str) -> Vec<String>
```

## Source
Lines 274–288 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [summarize_graphics_record](/crates/oxide-app/src/diagnostics/summarize_graphics_record.md) |
