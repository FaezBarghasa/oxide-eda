---
okf_version: "0.2"
type: Module
title: engine
description: "Rule engine: runs all registered built-in rules against an [`ErcContext`]"
resource: crates/oxide-erc/src/engine.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc/src/engine
language: rust
---

# engine

Rule engine: runs all registered built-in rules against an [`ErcContext`]

## Docstring

Rule engine: runs all registered built-in rules against an [`ErcContext`]
and returns a flat list of [`Diagnostic`]s in rule order.

DSL-compiled rules plug in via [`run_all_with_dsl`] using the [`EvalFn`]
type alias so the DSL crate never needs to depend back on the engine.

## Relationships

| Type | Target |
|------|--------|
| related | [run_all](/crates/oxide-erc/src/engine/run_all.md) |
| related | [run_all_with_dsl](/crates/oxide-erc/src/engine/run_all_with_dsl.md) |
