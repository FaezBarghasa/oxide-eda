---
okf_version: "0.2"
type: Class
title: FootprintLibSibling
description: "One row in the Footprint Library panel — a sibling `.snxfpt`"
resource: crates/oxide-app/src/panels/footprint_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_context/FootprintLibSibling
language: rust
---

# FootprintLibSibling

One row in the Footprint Library panel — a sibling `.snxfpt`

## Signature

```rust
pub struct FootprintLibSibling
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

One row in the Footprint Library panel — a sibling `.snxfpt`
living next to the active footprint inside the same `.snxlib`'s
`footprints/` directory.
[derive(Debug, Clone)]

## Methods

- `path`
- `display_name`
- `is_active`

## Source
Lines 428–438 in `crates/oxide-app/src/panels/footprint_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_context](/crates/oxide-app/src/panels/footprint_context.md) |
