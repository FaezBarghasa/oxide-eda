---
okf_version: "0.2"
type: Function
title: make_row
resource: crates/oxide-app/tests/support/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/support/mod/make_row
language: rust
---

# make_row

## Signature

```rust
fn make_row(index: usize, lib_id: Uuid, sym: Uuid, fpt: Uuid) -> ComponentRow
```

## Source
Lines 298–325 in `crates/oxide-app/tests/support/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [support](/crates/oxide-app/tests/support/mod.md) |
| calls | [sample_params](/crates/oxide-app/tests/support/mod/sample_params.md) |
| called_by | [append_component](/crates/oxide-app/tests/support/mod/append_component.md) |
| called_by | [generate_library](/crates/oxide-app/tests/support/mod/generate_library.md) |
