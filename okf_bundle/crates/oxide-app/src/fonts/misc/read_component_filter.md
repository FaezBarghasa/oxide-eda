---
okf_version: "0.2"
type: Function
title: read_component_filter
description: "Read the last-typed Components-panel filter, if any. Empty string"
resource: crates/oxide-app/src/fonts/misc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/fonts/misc/read_component_filter
language: rust
---

# read_component_filter

Read the last-typed Components-panel filter, if any. Empty string

## Signature

```rust
pub fn read_component_filter() -> String
```

## Visibility

- `pub`

## Docstring

Read the last-typed Components-panel filter, if any. Empty string
when missing or malformed — that's the same as a fresh session for
the panel.

## Source
Lines 41–50 in `crates/oxide-app/src/fonts/misc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [misc](/crates/oxide-app/src/fonts/misc.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
