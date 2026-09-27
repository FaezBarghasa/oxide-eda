---
okf_version: "0.2"
type: Function
title: parsed
description: "Parse both buffers as mm. `None` if either fails to parse."
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/parsed_1
language: rust
---

# parsed

Parse both buffers as mm. `None` if either fails to parse.

## Signature

```rust
pub fn parsed(&self) -> Option<(f64, f64)>
```

## Visibility

- `pub`

## Docstring

Parse both buffers as mm. `None` if either fails to parse.

## Source
Lines 69–74 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
