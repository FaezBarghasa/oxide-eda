---
okf_version: "0.2"
type: Function
title: insert_for_library
description: Add (or replace) a per-library override.
resource: crates/oxide-library/src/templates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/templates/insert_for_library
language: rust
---

# insert_for_library

Add (or replace) a per-library override.

## Signature

```rust
impl TemplateRegistry { pub fn insert_for_library(&mut self, library_id: Uuid, t: ParameterTemplate) }
```

## Visibility

- `pub`

## Docstring

Add (or replace) a per-library override.

## Source
Lines 160–162 in `crates/oxide-library/src/templates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [templates](/crates/oxide-library/src/templates.md) |
