---
okf_version: "0.2"
type: Function
title: ref_index
description: "Index into the flat `refs` array (and the `parent` union-find) for a"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/ref_index
language: rust
---

# ref_index

Index into the flat `refs` array (and the `parent` union-find) for a

## Signature

```rust
fn ref_index(seg: usize, is_start: bool) -> usize
```

## Docstring

Index into the flat `refs` array (and the `parent` union-find) for a
segment's start (`is_start = true`) or end (`is_start = false`)
endpoint. Start is ref `2*seg`, end is ref `2*seg + 1`.

## Source
Lines 503–505 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| called_by | [build_endpoint_clusters](/crates/oxide-library/src/primitive/symbol/chain/build_endpoint_clusters.md) |
