---
okf_version: "0.2"
type: Class
title: SplitCtx
description: Bundled split parameters threaded through constraint / reference
resource: crates/oxide-sketch/src/split/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/mod/SplitCtx
language: rust
---

# SplitCtx

Bundled split parameters threaded through constraint / reference

## Signature

```rust
struct SplitCtx
```

## Docstring

Bundled split parameters threaded through constraint / reference
carry-over so the per-kind helpers don't each take eight arguments.

## Methods

- `line`
- `line_a`
- `line_b`
- `start_xy`
- `dx`
- `dy`
- `len_sq`
- `t`

## Source
Lines 344–353 in `crates/oxide-sketch/src/split/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [split](/crates/oxide-sketch/src/split/mod.md) |
