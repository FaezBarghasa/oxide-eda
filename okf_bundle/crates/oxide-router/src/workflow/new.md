---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-router/src/workflow.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/workflow/new
language: rust
---

# new

## Signature

```rust
impl RoutingWorkflow { pub fn new(rules: Arc<ConstraintManager>, board: PcbBoard, nets: Vec<NetId>) -> Self }
```

## Visibility

- `pub`

## Source
Lines 32–40 in `crates/oxide-router/src/workflow.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [workflow](/crates/oxide-router/src/workflow.md) |
