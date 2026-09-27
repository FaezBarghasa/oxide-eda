---
okf_version: "0.2"
type: Class
title: SnxlibManifest
description: "Manifest header — everything in a `.snxlib` *except* the"
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod/SnxlibManifest
language: rust
---

# SnxlibManifest

Manifest header — everything in a `.snxlib` *except* the

## Signature

```rust
pub struct SnxlibManifest
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Manifest header — everything in a `.snxlib` *except* the
`[tables.*]` blocks.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `format`
- `library_id`
- `library`
- `mode`
- `workflow`
- `users`
- `classes`

## Source
Lines 56–79 in `crates/oxide-library/src/library_file/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_file](/crates/oxide-library/src/library_file/mod.md) |
