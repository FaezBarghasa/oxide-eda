---
okf_version: "0.2"
type: Function
title: lib_kind
description: ─────────────────────────────────────────────────────────────────────
resource: crates/oxide-bake/src/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/pad/lib_kind
language: rust
---

# lib_kind

─────────────────────────────────────────────────────────────────────

## Signature

```rust
fn lib_kind(k: PadKind, _warnings: &mut Vec<String>, _pad_number: &str) -> LibPadKind
```

## Docstring

─────────────────────────────────────────────────────────────────────
Helpers — kind / shape / layer mapping
─────────────────────────────────────────────────────────────────────

## Source
Lines 324–334 in `crates/oxide-bake/src/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-bake/src/pad.md) |
| called_by | [bake_one_pad](/crates/oxide-bake/src/pad/bake_one_pad.md) |
