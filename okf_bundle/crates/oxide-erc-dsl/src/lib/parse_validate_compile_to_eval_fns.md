---
okf_version: "0.2"
type: Function
title: parse_validate_compile_to_eval_fns
description: "Parse, validate, compile, and convert rules into engine evaluator closures."
resource: crates/oxide-erc-dsl/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc-dsl/src/lib/parse_validate_compile_to_eval_fns
language: rust
---

# parse_validate_compile_to_eval_fns

Parse, validate, compile, and convert rules into engine evaluator closures.

## Signature

```rust
pub fn parse_validate_compile_to_eval_fns(
    src: &str,
) -> Result<Vec<oxide_erc::engine::EvalFn>, Vec<DslError>>
```

## Visibility

- `pub`

## Docstring

Parse, validate, compile, and convert rules into engine evaluator closures.

## Source
Lines 47–52 in `crates/oxide-erc-dsl/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc-dsl/src/lib.md) |
| calls | [parse_validate_compile](/crates/oxide-erc-dsl/src/lib/parse_validate_compile.md) |
| calls | [to_eval_fns](/crates/oxide-erc-dsl/src/compiler/to_eval_fns.md) |
| called_by | [load_project_dsl_eval_fns](/crates/oxide-app/src/app/handlers/erc/erc_run/load_project_dsl_eval_fns.md) |
| called_by | [parse_validate_compile_to_eval_fns_runs_net_rule](/crates/oxide-erc-dsl/src/lib/parse_validate_compile_to_eval_fns_runs_net_rule.md) |
