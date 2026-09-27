---
okf_version: "0.2"
type: Function
title: validate_compatibility
description: Verifies interface compatibility between two connected harness ports.
resource: crates/oxide-net/src/harness.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:32:38Z"
concept_id: crates/oxide-net/src/harness/validate_compatibility_1
language: rust
---

# validate_compatibility

Verifies interface compatibility between two connected harness ports.

## Signature

```rust
pub fn validate_compatibility(&self, other: &Self) -> Result<(), String>
```

## Visibility

- `pub`

## Docstring

Verifies interface compatibility between two connected harness ports.

## Source
Lines 89–104 in `crates/oxide-net/src/harness.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [harness](/crates/oxide-net/src/harness.md) |
