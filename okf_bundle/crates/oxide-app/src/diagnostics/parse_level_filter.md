---
okf_version: "0.2"
type: Function
title: parse_level_filter
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/parse_level_filter
language: rust
---

# parse_level_filter

## Signature

```rust
fn parse_level_filter(value: &str) -> Option<LevelFilter>
```

## Source
Lines 156–178 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| calls | [parse_level](/crates/oxide-app/src/diagnostics/parse_level.md) |
| called_by | [resolve_configured_level](/crates/oxide-app/src/diagnostics/resolve_configured_level.md) |
