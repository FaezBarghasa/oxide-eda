---
okf_version: "0.2"
type: Class
title: Rotatable2d
description: Adapter trait for per-object rotation integration.
resource: crates/oxide-types/src/rotation2d.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/rotation2d/Rotatable2d
language: rust
---

# Rotatable2d

Adapter trait for per-object rotation integration.

## Signature

```rust
pub trait Rotatable2d
```

## Visibility

- `pub`

## Docstring

Adapter trait for per-object rotation integration.

This keeps the rotation math centralized while allowing each object type
to map its own pose/anchor conventions when callers integrate it.

## Source
Lines 75–79 in `crates/oxide-types/src/rotation2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation2d](/crates/oxide-types/src/rotation2d.md) |
