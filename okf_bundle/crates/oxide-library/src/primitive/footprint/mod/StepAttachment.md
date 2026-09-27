---
okf_version: "0.2"
type: Class
title: StepAttachment
description: Optional STEP/WRL attachment for mech-CAD export. Content-hashed so two
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/StepAttachment
language: rust
---

# StepAttachment

Optional STEP/WRL attachment for mech-CAD export. Content-hashed so two

## Signature

```rust
pub struct StepAttachment
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Optional STEP/WRL attachment for mech-CAD export. Content-hashed so two
MPNs with identical STEP geometry de-duplicate to one file in
`mylib.snxlib/step/<sha256>.step`.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `content_hash`
- `filename`
- `offset_xyz`
- `rotation_xyz`

## Source
Lines 292–301 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
