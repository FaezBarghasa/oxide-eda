---
okf_version: "0.2"
type: Class
title: ConstellationPoint
description: Constellation Point with ideal reference and noisy received coordinates.
resource: crates/oxide-rf/src/constellation.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:26:57Z"
concept_id: crates/oxide-rf/src/constellation/ConstellationPoint
language: rust
---

# ConstellationPoint

Constellation Point with ideal reference and noisy received coordinates.

## Signature

```rust
pub struct ConstellationPoint
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Constellation Point with ideal reference and noisy received coordinates.
[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]

## Methods

- `received`
- `ideal`
- `error_vector`

## Source
Lines 8–12 in `crates/oxide-rf/src/constellation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constellation](/crates/oxide-rf/src/constellation.md) |
