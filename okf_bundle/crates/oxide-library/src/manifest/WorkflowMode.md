---
okf_version: "0.2"
type: Class
title: WorkflowMode
description: Per-library workflow mode — controls which versioning + cascade
resource: crates/oxide-library/src/manifest.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/manifest/WorkflowMode
language: rust
---

# WorkflowMode

Per-library workflow mode — controls which versioning + cascade

## Signature

```rust
pub enum WorkflowMode
```

## Decorators

- `derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)`
- `serde(rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Per-library workflow mode — controls which versioning + cascade
gates are visible in the UI. Stored as a TOML string token.
[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
[serde(rename_all = "snake_case")]

## Source
Lines 67–80 in `crates/oxide-library/src/manifest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manifest](/crates/oxide-library/src/manifest.md) |
