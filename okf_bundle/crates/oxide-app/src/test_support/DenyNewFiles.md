---
okf_version: "0.2"
type: Class
title: DenyNewFiles
description: "RAII guard: while alive, `dir` rejects new-file creation. Restores"
resource: crates/oxide-app/src/test_support.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/test_support/DenyNewFiles
language: rust
---

# DenyNewFiles

RAII guard: while alive, `dir` rejects new-file creation. Restores

## Signature

```rust
pub struct DenyNewFiles
```

## Visibility

- `pub`

## Docstring

RAII guard: while alive, `dir` rejects new-file creation. Restores
permissions on drop so the tempdir this is used inside still cleans
up normally.

## Methods

- `dir`

## Source
Lines 18–20 in `crates/oxide-app/src/test_support.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [test_support](/crates/oxide-app/src/test_support.md) |
