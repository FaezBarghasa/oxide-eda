---
okf_version: "0.2"
type: Function
title: execute
description: "Execute all routing phases: Preparation, Global Topological Routing, Detailed Routing, Optimization."
resource: crates/oxide-router/src/workflow.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/workflow/execute
language: rust
---

# execute

Execute all routing phases: Preparation, Global Topological Routing, Detailed Routing, Optimization.

## Signature

```rust
impl RoutingWorkflow { pub fn execute(&mut self) -> WorkflowResult }
```

## Visibility

- `pub`

## Docstring

Execute all routing phases: Preparation, Global Topological Routing, Detailed Routing, Optimization.

## Source
Lines 43–107 in `crates/oxide-router/src/workflow.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [workflow](/crates/oxide-router/src/workflow.md) |
