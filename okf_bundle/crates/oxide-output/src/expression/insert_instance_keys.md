---
okf_version: "0.2"
type: Function
title: insert_instance_keys
resource: crates/oxide-output/src/expression.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T19:55:16Z"
concept_id: crates/oxide-output/src/expression/insert_instance_keys
language: rust
---

# insert_instance_keys

## Signature

```rust
fn insert_instance_keys(
    out: &mut HashMap<String, String>,
    instance: &SymbolInstance,
    reference: &str,
)
```

## Source
Lines 59–74 in `crates/oxide-output/src/expression.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [expression](/crates/oxide-output/src/expression.md) |
| called_by | [build_global_refdes_lookup](/crates/oxide-output/src/expression/build_global_refdes_lookup.md) |
