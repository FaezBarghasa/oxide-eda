---
okf_version: "0.2"
type: Function
title: verify
description: Fail loudly if the generated library does not open and enumerate
resource: crates/oxide-app/tests/support/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/support/mod/verify
language: rust
---

# verify

Fail loudly if the generated library does not open and enumerate

## Signature

```rust
fn verify(snxlib: &Path, scale: &Scale) -> Result<(), LibraryError>
```

## Docstring

Fail loudly if the generated library does not open and enumerate
exactly what was written.

## Source
Lines 483–513 in `crates/oxide-app/tests/support/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [support](/crates/oxide-app/tests/support/mod.md) |
| called_by | [generate_library](/crates/oxide-app/tests/support/mod/generate_library.md) |
