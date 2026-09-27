---
okf_version: "0.2"
type: Function
title: parse_level
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/parse_level
language: rust
---

# parse_level

## Signature

```rust
fn parse_level(level: &str) -> Option<LevelFilter>
```

## Source
Lines 180–190 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| called_by | [parse_level_filter](/crates/oxide-app/src/diagnostics/parse_level_filter.md) |
