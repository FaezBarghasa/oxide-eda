---
okf_version: "0.2"
type: Class
title: SimError
description: Error types occurring during circuit simulation.
resource: crates/oxide-sim/src/simulator/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:13:06Z"
concept_id: crates/oxide-sim/src/simulator/mod/SimError
language: rust
---

# SimError

Error types occurring during circuit simulation.

## Signature

```rust
pub enum SimError
```

## Decorators

- `derive(Debug, Error)`

## Visibility

- `pub`

## Docstring

Error types occurring during circuit simulation.
[derive(Debug, Error)]

## Methods

- `name`
- `details`
- `exit_code`
- `message`

## Source
Lines 15–39 in `crates/oxide-sim/src/simulator/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simulator](/crates/oxide-sim/src/simulator/mod.md) |
