---
okf_version: "0.2"
type: Function
title: first_file_len
resource: crates/oxide-app/tests/support/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/support/mod/first_file_len
language: rust
---

# first_file_len

## Signature

```rust
fn first_file_len(dir: &Path) -> u64
```

## Source
Lines 525–535 in `crates/oxide-app/tests/support/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [support](/crates/oxide-app/tests/support/mod.md) |
| calls | [metadata](/crates/oxide-output/src/substitution/metadata.md) |
| called_by | [primitive_file_sizes](/crates/oxide-app/tests/support/mod/primitive_file_sizes.md) |
