---
okf_version: "0.2"
type: Function
title: intersects
description: "True when any bit of `mask` is also set in `self`."
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/intersects
language: rust
---

# intersects

True when any bit of `mask` is also set in `self`.

## Signature

```rust
impl RenderInvalidation { pub fn intersects(self, mask: Self) -> bool }
```

## Visibility

- `pub`

## Docstring

True when any bit of `mask` is also set in `self`.

## Source
Lines 53–55 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
