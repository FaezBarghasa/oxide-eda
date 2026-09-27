---
okf_version: "0.2"
type: Class
title: ExportContext
description: Everything an exporter needs to know about the project being exported.
resource: crates/oxide-output/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:20:47Z"
concept_id: crates/oxide-output/src/lib/ExportContext
language: rust
---

# ExportContext

Everything an exporter needs to know about the project being exported.

## Signature

```rust
pub struct ExportContext
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Everything an exporter needs to know about the project being exported.

The app layer builds this from `DocumentState` at export time — exporters
never touch the live application state directly.
[derive(Debug, Clone)]

## Methods

- `sheets`
- `metadata`
- `netlist`

## Source
Lines 72–81 in `crates/oxide-output/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-output/src/lib.md) |
