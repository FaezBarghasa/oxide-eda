---
okf_version: "0.2"
type: Function
title: lookup_ci
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup/lookup_ci
language: rust
---

# lookup_ci

## Signature

```rust
fn lookup_ci(map: Option<&HashMap<String, String>>, key: &str) -> Option<String>
```

## Source
Lines 503–511 in `crates/oxide-types/src/markup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [markup](/crates/oxide-types/src/markup.md) |
| called_by | [eval_at_expression](/crates/oxide-types/src/markup/eval_at_expression.md) |
| called_by | [eval_dollar_expression](/crates/oxide-types/src/markup/eval_dollar_expression.md) |
| called_by | [eval_net_name](/crates/oxide-types/src/markup/eval_net_name.md) |
