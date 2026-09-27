---
okf_version: "0.2"
type: Function
title: workflow_mode_round_trips
description: Workflow mode round-trips through TOML — Stage 13 of
resource: crates/oxide-library/src/manifest.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/manifest/workflow_mode_round_trips
language: rust
---

# workflow_mode_round_trips

Workflow mode round-trips through TOML — Stage 13 of

## Signature

```rust
fn workflow_mode_round_trips()
```

## Decorators

- `test`

## Docstring

Workflow mode round-trips through TOML — Stage 13 of
`v0.9-snxlib-as-file-plan.md`. The picker defaults to
`Personal` for new libraries and gates the §3.5 versioning UI
(released flag, bump dialog, cascade modal) when set to
`Team`.
[test]

## Source
Lines 199–221 in `crates/oxide-library/src/manifest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manifest](/crates/oxide-library/src/manifest.md) |
