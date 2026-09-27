---
okf_version: "0.2"
type: Function
title: export
description: "An export deliverable (PDF, netlist, BOM) could not be produced."
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/export
language: rust
---

# export

An export deliverable (PDF, netlist, BOM) could not be produced.

## Signature

```rust
impl ErrorNotice { pub fn export(detail: impl Into<String>) -> Self }
```

## Visibility

- `pub`

## Docstring

An export deliverable (PDF, netlist, BOM) could not be produced.

## Source
Lines 159–164 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
