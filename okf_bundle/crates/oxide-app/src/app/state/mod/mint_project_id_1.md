---
okf_version: "0.2"
type: Function
title: mint_project_id
description: "Mint a fresh `ProjectId` and bump the counter. Never reuses ids."
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/mint_project_id_1
language: rust
---

# mint_project_id

Mint a fresh `ProjectId` and bump the counter. Never reuses ids.

## Signature

```rust
pub fn mint_project_id(&mut self) -> ProjectId
```

## Visibility

- `pub`

## Docstring

Mint a fresh `ProjectId` and bump the counter. Never reuses ids.

## Source
Lines 635–639 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
| calls | [ProjectId](/crates/oxide-app/src/app/state/mod/ProjectId.md) |
