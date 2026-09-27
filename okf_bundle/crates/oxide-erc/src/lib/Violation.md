---
okf_version: "0.2"
type: Class
title: Violation
description: "A concrete violation emitted by a rule run. `location` is in world-space"
resource: crates/oxide-erc/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc/src/lib/Violation
language: rust
---

# Violation

A concrete violation emitted by a rule run. `location` is in world-space

## Signature

```rust
pub struct Violation
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

A concrete violation emitted by a rule run. `location` is in world-space
mm; the app uses it to centre the canvas when the user clicks the message.
[derive(Debug, Clone)]

## Methods

- `rule`
- `severity`
- `message`
- `location`
- `primary`
- `peer`

## Source
Lines 118–127 in `crates/oxide-erc/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc/src/lib.md) |
