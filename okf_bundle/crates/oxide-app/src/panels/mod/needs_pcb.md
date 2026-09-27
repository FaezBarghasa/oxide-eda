---
okf_version: "0.2"
type: Function
title: needs_pcb
description: Whether this panel requires a PCB document to be open.
resource: crates/oxide-app/src/panels/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:31:39Z"
concept_id: crates/oxide-app/src/panels/mod/needs_pcb
language: rust
---

# needs_pcb

Whether this panel requires a PCB document to be open.

## Signature

```rust
impl PanelKind { pub fn needs_pcb(self) -> bool }
```

## Visibility

- `pub`

## Docstring

Whether this panel requires a PCB document to be open.

## Source
Lines 210–212 in `crates/oxide-app/src/panels/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [panels](/crates/oxide-app/src/panels/mod.md) |
