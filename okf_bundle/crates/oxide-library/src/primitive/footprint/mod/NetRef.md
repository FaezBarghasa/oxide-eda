---
okf_version: "0.2"
type: Class
title: NetRef
description: "Net reference — string for now, will become a UUID once nets are"
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/NetRef
language: rust
---

# NetRef

Net reference — string for now, will become a UUID once nets are

## Signature

```rust
pub struct NetRef
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(transparent)`

## Visibility

- `pub`

## Docstring

Net reference — string for now, will become a UUID once nets are
first-class library citizens (v0.16+). v0.14 introduces this as a
thin wrapper so future migration can be one-shot.
[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(transparent)]

## Source
Lines 144–144 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
| called_by | [bake_pours](/crates/oxide-bake/src/pour/bake_pours.md) |
