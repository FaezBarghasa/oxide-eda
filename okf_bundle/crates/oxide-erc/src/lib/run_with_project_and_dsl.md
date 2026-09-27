---
okf_version: "0.2"
type: Function
title: run_with_project_and_dsl
description: Run project-scoped ERC with built-in rules plus caller-provided DSL rules.
resource: crates/oxide-erc/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc/src/lib/run_with_project_and_dsl
language: rust
---

# run_with_project_and_dsl

Run project-scoped ERC with built-in rules plus caller-provided DSL rules.

## Signature

```rust
pub fn run_with_project_and_dsl(
    snapshot: &SchematicSheet,
    resolved: &std::collections::HashMap<String, oxide_net::SheetKey>,
    sheets: &std::collections::HashMap<oxide_net::SheetKey, SchematicSheet>,
    dsl_rules: &[engine::EvalFn],
) -> Vec<Violation>
```

## Visibility

- `pub`

## Docstring

Run project-scoped ERC with built-in rules plus caller-provided DSL rules.
See [`run_with_project`] for what `resolved` / `sheets` must be.

## Source
Lines 172–183 in `crates/oxide-erc/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc/src/lib.md) |
| calls | [run_all_with_dsl](/crates/oxide-erc/src/engine/run_all_with_dsl.md) |
| called_by | [handle_run_erc](/crates/oxide-app/src/app/handlers/erc/erc_run/handle_run_erc.md) |
