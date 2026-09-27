---
okf_version: "0.2"
type: Class
title: LocalColorSlot
description: "Per-tab state for an open `.snxsym` document. Symbol editing"
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/LocalColorSlot
language: rust
---

# LocalColorSlot

Per-tab state for an open `.snxsym` document. Symbol editing

## Signature

```rust
pub enum LocalColorSlot
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Per-tab state for an open `.snxsym` document. Symbol editing
happens standalone (not embedded in the Component Preview tab),
keyed by file path so the same primitive can be edited without a
hosting `ComponentRow`.

The editor reuses the existing
[`crate::library::editor::symbol::canvas::SymbolCanvas`] program for
pin layout + the existing
[`crate::library::editor::symbol::state`] mutation helpers, so the
behaviour matches the in-Component Editor experience verbatim.
One of the three symbol-level local-colour slots. Each can carry an
independent RGBA override (or inherit from the sheet palette).
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 204–208 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
