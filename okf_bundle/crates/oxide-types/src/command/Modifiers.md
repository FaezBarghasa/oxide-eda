---
okf_version: "0.2"
type: Class
title: Modifiers
description: "Modifier keys (Ctrl/Cmd, Shift, Alt)."
resource: crates/oxide-types/src/command.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:47:18Z"
concept_id: crates/oxide-types/src/command/Modifiers
language: rust
---

# Modifiers

Modifier keys (Ctrl/Cmd, Shift, Alt).

## Signature

```rust
pub struct Modifiers
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)`

## Visibility

- `pub`

## Docstring

Modifier keys (Ctrl/Cmd, Shift, Alt).
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]

## Methods

- `ctrl`
- `shift`
- `alt`
- `meta`

## Source
Lines 285–290 in `crates/oxide-types/src/command.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command](/crates/oxide-types/src/command.md) |
