---
okf_version: "0.2"
type: Function
title: refresh_active_erc_from_cache
description: "Repoint `erc_violations` + canvas markers at whatever the"
resource: crates/oxide-app/src/app/handlers/erc/erc_run.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/handlers/erc/erc_run/refresh_active_erc_from_cache_1
language: rust
---

# refresh_active_erc_from_cache

Repoint `erc_violations` + canvas markers at whatever the

## Signature

```rust
pub(crate) fn refresh_active_erc_from_cache(
        &mut self,
        active_path: Option<&std::path::PathBuf>,
    )
```

## Visibility

- `pub(crate)`

## Docstring

Repoint `erc_violations` + canvas markers at whatever the
per-sheet cache holds for `active_path`. Empty vec when the
sheet has never had ERC run, which is the right behaviour
pre-Run-ERC.

## Source
Lines 202–227 in `crates/oxide-app/src/app/handlers/erc/erc_run.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc_run](/crates/oxide-app/src/app/handlers/erc/erc_run.md) |
