---
okf_version: "0.2"
type: Function
title: refresh_content_hash
description: Recompute and store the content hash from the canonical view.
resource: crates/oxide-library/src/component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/component/refresh_content_hash
language: rust
---

# refresh_content_hash

Recompute and store the content hash from the canonical view.

## Signature

```rust
impl ComponentRow { pub fn refresh_content_hash(&mut self) -> Result<(), crate::adapter::LibraryError> }
```

## Visibility

- `pub`

## Docstring

Recompute and store the content hash from the canonical view.

Returns `LibraryError::Backend` when the row contains a non-finite
float (`NaN` / `±Infinity`) anywhere reached by the canonical view —
`serde_json` can't encode those, and panicking on save would be an
availability bug. See [`crate::hash::hash_row_content`].

## Source
Lines 186–189 in `crates/oxide-library/src/component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component](/crates/oxide-library/src/component.md) |
| calls | [hash_row_content](/crates/oxide-library/src/hash/hash_row_content.md) |
