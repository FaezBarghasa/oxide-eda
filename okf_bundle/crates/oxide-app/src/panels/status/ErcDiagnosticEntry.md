---
okf_version: "0.2"
type: Class
title: ErcDiagnosticEntry
description: Flattened ERC diagnostic row for the ERC panel.
resource: crates/oxide-app/src/panels/status.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/status/ErcDiagnosticEntry
language: rust
---

# ErcDiagnosticEntry

Flattened ERC diagnostic row for the ERC panel.

## Signature

```rust
pub struct ErcDiagnosticEntry
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Flattened ERC diagnostic row for the ERC panel.

The app flattens per-sheet ERC caches into this list so the panel can
present one navigable table across the entire project.
[derive(Debug, Clone)]

## Methods

- `global_index`
- `sheet_name`
- `sheet_path`
- `severity`
- `rule_label`
- `rule_kind`
- `message`
- `world_x`
- `world_y`
- `select`

## Source
Lines 10–26 in `crates/oxide-app/src/panels/status.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [status](/crates/oxide-app/src/panels/status.md) |
