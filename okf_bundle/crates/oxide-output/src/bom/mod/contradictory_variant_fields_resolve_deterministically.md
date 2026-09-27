---
okf_version: "0.2"
type: Function
title: contradictory_variant_fields_resolve_deterministically
description: "`symbol.fields` is a `HashMap`, so a symbol carrying contradictory"
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/contradictory_variant_fields_resolve_deterministically
language: rust
---

# contradictory_variant_fields_resolve_deterministically

`symbol.fields` is a `HashMap`, so a symbol carrying contradictory

## Signature

```rust
fn contradictory_variant_fields_resolve_deterministically()
```

## Decorators

- `test`

## Docstring

`symbol.fields` is a `HashMap`, so a symbol carrying contradictory
variant entries used to resolve by whichever key the allocator handed
over first — the same file could export a different fit state run to
run. Fitted wins over DNP, deterministically. Rebuilt per iteration
because `RandomState` re-seeds per map, not per process.
[test]

## Source
Lines 588–600 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| calls | [test_symbol](/crates/oxide-output/src/bom/mod/test_symbol.md) |
