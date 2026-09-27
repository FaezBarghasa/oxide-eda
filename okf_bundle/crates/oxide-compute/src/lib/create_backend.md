---
okf_version: "0.2"
type: Function
title: create_backend
description: Factory function to select and initialize the best available compute backend
resource: crates/oxide-compute/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:42:43Z"
concept_id: crates/oxide-compute/src/lib/create_backend
language: rust
---

# create_backend

Factory function to select and initialize the best available compute backend

## Signature

```rust
pub fn create_backend(preference: BackendPreference) -> Box<dyn ComputeBackend>
```

## Visibility

- `pub`

## Docstring

Factory function to select and initialize the best available compute backend

## Source
Lines 22–48 in `crates/oxide-compute/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-compute/src/lib.md) |
| called_by | [test_auto_backend_factory](/crates/oxide-compute/tests/compute_tests/test_auto_backend_factory.md) |
