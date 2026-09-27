---
okf_version: "0.2"
type: Class
title: ProjectMsg
description: Project lifecycle message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/ProjectMsg
language: rust
---

# ProjectMsg

Project lifecycle message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum ProjectMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Project lifecycle message family (ADR-0001 D3). Namespaced under
`Message::Project` and routed to `dispatch_project_message`.
[derive(Debug, Clone)]

## Methods

- `project_idx`
- `paths`
- `project_idx`
- `path`
- `project_root`
- `rel_path`
- `result`

## Source
Lines 291–330 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
