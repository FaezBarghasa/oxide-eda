---
okf_version: "0.2"
type: Module
title: lib
description: "Electrical Rules Check. Runs on a [`SchematicSheet`] and returns a"
resource: crates/oxide-erc/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc/src/lib
language: rust
---

# lib

Electrical Rules Check. Runs on a [`SchematicSheet`] and returns a

## Docstring

Electrical Rules Check. Runs on a [`SchematicSheet`] and returns a
list of [`Violation`]s. Internally the sheet is projected into an
[`ErcContext`] first, so rule logic stays independent from renderer internals.

# Architecture

```text
SchematicSheet
↓  (projection)
ErcContext
↓  (engine::run_all)
Vec<Diagnostic>
↓  (From<Diagnostic>)
Vec<Violation>   ← public API
```

ERC reads from `oxide_types::SchematicSheet` directly.

## Relationships

| Type | Target |
|------|--------|
| related | [RuleKind](/crates/oxide-erc/src/lib/RuleKind.md) |
| related | [label](/crates/oxide-erc/src/lib/label.md) |
| related | [default_severity](/crates/oxide-erc/src/lib/default_severity.md) |
| related | [label](/crates/oxide-erc/src/lib/label.md) |
| related | [default_severity](/crates/oxide-erc/src/lib/default_severity.md) |
| related | [Severity](/crates/oxide-erc/src/lib/Severity.md) |
| related | [Violation](/crates/oxide-erc/src/lib/Violation.md) |
| related | [run](/crates/oxide-erc/src/lib/run.md) |
| related | [run_with_dsl](/crates/oxide-erc/src/lib/run_with_dsl.md) |
| related | [run_with_project](/crates/oxide-erc/src/lib/run_with_project.md) |
| related | [run_with_project_and_dsl](/crates/oxide-erc/src/lib/run_with_project_and_dsl.md) |
| related | [sel](/crates/oxide-erc/src/lib/sel.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
