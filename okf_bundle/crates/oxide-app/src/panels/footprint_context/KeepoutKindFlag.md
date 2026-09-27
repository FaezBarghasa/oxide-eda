---
okf_version: "0.2"
type: Class
title: KeepoutKindFlag
description: "v0.16.4 — discrete bit identifier for [`KeepoutSummary`] flags."
resource: crates/oxide-app/src/panels/footprint_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_context/KeepoutKindFlag
language: rust
---

# KeepoutKindFlag

v0.16.4 — discrete bit identifier for [`KeepoutSummary`] flags.

## Signature

```rust
pub enum KeepoutKindFlag
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

v0.16.4 — discrete bit identifier for [`KeepoutSummary`] flags.
PanelMsg-friendly so the Keepout sub-form's checklist can carry
"which flag" without smuggling a closure.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 386–393 in `crates/oxide-app/src/panels/footprint_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_context](/crates/oxide-app/src/panels/footprint_context.md) |
