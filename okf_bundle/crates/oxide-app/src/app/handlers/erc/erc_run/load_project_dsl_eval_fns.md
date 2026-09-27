---
okf_version: "0.2"
type: Function
title: load_project_dsl_eval_fns
resource: crates/oxide-app/src/app/handlers/erc/erc_run.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/handlers/erc/erc_run/load_project_dsl_eval_fns
language: rust
---

# load_project_dsl_eval_fns

## Signature

```rust
impl Oxide { fn load_project_dsl_eval_fns(&self) -> Option<Vec<oxide_erc::engine::EvalFn>> }
```

## Source
Lines 163–196 in `crates/oxide-app/src/app/handlers/erc/erc_run.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc_run](/crates/oxide-app/src/app/handlers/erc/erc_run.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
| calls | [parse_validate_compile_to_eval_fns](/crates/oxide-erc-dsl/src/lib/parse_validate_compile_to_eval_fns.md) |
