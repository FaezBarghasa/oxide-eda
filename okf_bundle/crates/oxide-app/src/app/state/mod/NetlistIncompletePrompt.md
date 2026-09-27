---
okf_version: "0.2"
type: Class
title: NetlistIncompletePrompt
description: "#431 — pending \"Export anyway (incomplete)?\" prompt for netlist export."
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/NetlistIncompletePrompt
language: rust
---

# NetlistIncompletePrompt

#431 — pending "Export anyway (incomplete)?" prompt for netlist export.

## Signature

```rust
pub struct NetlistIncompletePrompt
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

#431 — pending "Export anyway (incomplete)?" prompt for netlist export.

Raised (instead of a dead-end error) when `File ▸ Export Netlist` derives a
netlist that does not cover the whole project. Holds the picked `save_path`
and the omitted-page `messages` so the two prompt actions can act without
re-deriving: "Export anyway" writes the partial `.net` with these messages
recorded in its INCOMPLETE header comment; "Cancel" writes nothing. The
refusal stays the default — nothing reaches disk until the user acts.
[derive(Debug, Clone)]

## Methods

- `save_path`
- `messages`
- `ctx`

## Source
Lines 301–314 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
