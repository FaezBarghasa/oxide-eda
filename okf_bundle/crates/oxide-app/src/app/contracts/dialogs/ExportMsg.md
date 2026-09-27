---
okf_version: "0.2"
type: Class
title: ExportMsg
description: Export subsystem message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/ExportMsg
language: rust
---

# ExportMsg

Export subsystem message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum ExportMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Export subsystem message family (ADR-0001 D3). Namespaced under
`Message::Export` and routed to `dispatch_export_message`. Covers
the PDF / netlist / BOM export lifecycle plus the export-error
modal dismiss.
[derive(Debug, Clone)]

## Source
Lines 196–219 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
