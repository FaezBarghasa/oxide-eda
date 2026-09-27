---
okf_version: "0.2"
type: Function
title: symbol_eval_variables
resource: crates/oxide-output/src/svg/symbols.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/symbols/symbol_eval_variables
language: rust
---

# symbol_eval_variables

## Signature

```rust
pub(super) fn symbol_eval_variables(sym: &Symbol) -> HashMap<String, String>
```

## Visibility

- `pub(super)`

## Source
Lines 430–444 in `crates/oxide-output/src/svg/symbols.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbols](/crates/oxide-output/src/svg/symbols.md) |
| called_by | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
