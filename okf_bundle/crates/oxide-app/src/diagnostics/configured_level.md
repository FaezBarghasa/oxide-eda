---
okf_version: "0.2"
type: Function
title: configured_level
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/configured_level
language: rust
---

# configured_level

## Signature

```rust
pub fn configured_level() -> LevelFilter
```

## Visibility

- `pub`

## Source
Lines 89–91 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| called_by | [configured_level_label](/crates/oxide-app/src/diagnostics/configured_level_label.md) |
| called_by | [init_logging](/crates/oxide-app/src/diagnostics/init_logging.md) |
