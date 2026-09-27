---
okf_version: "0.2"
type: Class
title: TabDocument
description: Per-tab auxiliary document payload. Schematic tabs keep their
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/TabDocument
language: rust
---

# TabDocument

Per-tab auxiliary document payload. Schematic tabs keep their

## Signature

```rust
pub enum TabDocument
```

## Decorators

- `derive(Debug)`

## Visibility

- `pub`

## Docstring

Per-tab auxiliary document payload. Schematic tabs keep their
engine in `DocumentState::engines` (keyed by path) rather than in
this enum; currently only PCB tabs carry a document here. Kept as
an enum so future tab kinds (symbol editor, footprint editor, 3D
viewer) can slot in without reshaping callers.
[derive(Debug)]

## Source
Lines 160–162 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
