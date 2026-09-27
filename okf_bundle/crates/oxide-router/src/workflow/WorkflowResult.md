---
okf_version: "0.2"
type: Class
title: WorkflowResult
description: End-to-end execution result of a complete board routing workflow.
resource: crates/oxide-router/src/workflow.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/workflow/WorkflowResult
language: rust
---

# WorkflowResult

End-to-end execution result of a complete board routing workflow.

## Signature

```rust
pub struct WorkflowResult
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

End-to-end execution result of a complete board routing workflow.
[derive(Debug, Clone)]

## Methods

- `total_nets`
- `routed_nets`
- `failed_nets`
- `total_length`

## Source
Lines 15–20 in `crates/oxide-router/src/workflow.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [workflow](/crates/oxide-router/src/workflow.md) |
