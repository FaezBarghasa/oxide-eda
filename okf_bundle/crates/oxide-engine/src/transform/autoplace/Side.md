---
okf_version: "0.2"
type: Class
title: Side
description: 4. Pick the side. Score = pin_count + anchor_penalty.
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/Side
language: rust
---

# Side

4. Pick the side. Score = pin_count + anchor_penalty.

## Signature

```rust
enum Side
```

## Decorators

- `derive(Clone, Copy)`

## Docstring

4. Pick the side. Score = pin_count + anchor_penalty.
Tie-break order is Oxide-original: Bottom > Top > Left > Right.
[derive(Clone, Copy)]

## Source
Lines 152–157 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
