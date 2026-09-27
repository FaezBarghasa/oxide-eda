---
okf_version: "0.2"
type: Class
title: NewTableDraft
description: "Inline form state for \"+ New Table…\" inside the New Component"
resource: crates/oxide-app/src/library/state/preview.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/state/preview/NewTableDraft
language: rust
---

# NewTableDraft

Inline form state for "+ New Table…" inside the New Component

## Signature

```rust
pub struct NewTableDraft
```

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

Inline form state for "+ New Table…" inside the New Component
modal — collects the name (and validation error if any) before the
dispatcher calls `create_empty_table` on the active library.
[derive(Debug, Clone, Default)]

## Methods

- `name`
- `error`

## Source
Lines 64–67 in `crates/oxide-app/src/library/state/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/state/preview.md) |
