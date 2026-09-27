---
okf_version: "0.2"
type: Function
title: has_populated_field
resource: crates/oxide-bom/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bom"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bom/src/lib/has_populated_field
language: rust
---

# has_populated_field

## Signature

```rust
fn has_populated_field(component: &BomComponent, candidates: &[&str]) -> bool
```

## Source
Lines 291–298 in `crates/oxide-bom/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-bom/src/lib.md) |
| called_by | [validate_table](/crates/oxide-bom/src/lib/validate_table.md) |
