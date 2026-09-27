---
okf_version: "0.2"
type: Function
title: build_global_refdes_lookup
resource: crates/oxide-output/src/expression.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T19:55:16Z"
concept_id: crates/oxide-output/src/expression/build_global_refdes_lookup
language: rust
---

# build_global_refdes_lookup

## Signature

```rust
fn build_global_refdes_lookup(sheets: &[SheetSnapshot]) -> HashMap<String, String>
```

## Source
Lines 38–57 in `crates/oxide-output/src/expression.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [expression](/crates/oxide-output/src/expression.md) |
| calls | [insert_instance_keys](/crates/oxide-output/src/expression/insert_instance_keys.md) |
| called_by | [build_expression_tables](/crates/oxide-output/src/expression/build_expression_tables.md) |
