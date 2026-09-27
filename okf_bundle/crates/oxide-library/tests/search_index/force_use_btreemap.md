---
okf_version: "0.2"
type: Function
title: _force_use_btreemap
description: "Defensive: keep an unused `BTreeMap` import in scope so cargo doesn't warn"
resource: crates/oxide-library/tests/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/search_index/force_use_btreemap
language: rust
---

# _force_use_btreemap

Defensive: keep an unused `BTreeMap` import in scope so cargo doesn't warn

## Signature

```rust
fn _force_use_btreemap() -> BTreeMap<String, String>
```

## Docstring

Defensive: keep an unused `BTreeMap` import in scope so cargo doesn't warn
when the test file evolves; suppression rather than removal because adapter
tests often re-introduce these collections.

## Source
Lines 460–462 in `crates/oxide-library/tests/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/tests/search_index.md) |
