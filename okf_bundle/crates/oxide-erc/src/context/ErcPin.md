---
okf_version: "0.2"
type: Class
title: ErcPin
description: "A single pin instance in world-space, ready for rule evaluation."
resource: crates/oxide-erc/src/context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/context/ErcPin
language: rust
---

# ErcPin

A single pin instance in world-space, ready for rule evaluation.

## Signature

```rust
pub struct ErcPin
```

## Decorators

- `derive(Debug, Clone, Copy)`

## Visibility

- `pub`

## Docstring

A single pin instance in world-space, ready for rule evaluation.
[derive(Debug, Clone, Copy)]

## Methods

- `world_pos`
- `electrical_type`
- `required`
- `connected`

## Source
Lines 58–67 in `crates/oxide-erc/src/context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context](/crates/oxide-erc/src/context.md) |
