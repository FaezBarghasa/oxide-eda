---
okf_version: "0.2"
type: Function
title: hash_artifact
description: Computes content hash (SHA-256) of raw artifact bytes for deduplication.
resource: crates/oxide-library/src/harvester/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:01:27Z"
concept_id: crates/oxide-library/src/harvester/cascade/hash_artifact
language: rust
---

# hash_artifact

Computes content hash (SHA-256) of raw artifact bytes for deduplication.

## Signature

```rust
impl HarvesterCascade { pub fn hash_artifact(bytes: &[u8]) -> String }
```

## Visibility

- `pub`

## Docstring

Computes content hash (SHA-256) of raw artifact bytes for deduplication.

## Source
Lines 34–39 in `crates/oxide-library/src/harvester/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-library/src/harvester/cascade.md) |
