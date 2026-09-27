---
okf_version: "0.2"
type: Class
title: SelectionFilterCustomState
description: v0.18.14.1 — Custom Selection Filter modal draft state. Mirrors
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/SelectionFilterCustomState
language: rust
---

# SelectionFilterCustomState

v0.18.14.1 — Custom Selection Filter modal draft state. Mirrors

## Signature

```rust
pub struct SelectionFilterCustomState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

v0.18.14.1 — Custom Selection Filter modal draft state. Mirrors
`SelectionFilter` from `library::editor::footprint::state` so
the user can flip flags without touching the live editor until
they hit Apply.
[derive(Debug, Clone)]

## Methods

- `pads`
- `tracks`
- `arcs`
- `pours`
- `bodies_3d`
- `keepouts`
- `cutouts`
- `texts`
- `vias`
- `regions`
- `fills`
- `other`

## Source
Lines 90–103 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
