---
okf_version: "0.2"
type: Class
title: TestpointFlags
description: Test-point participation flags. Each bool selects one of the four
resource: crates/oxide-sketch/src/attr.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-sketch/src/attr/TestpointFlags
language: rust
---

# TestpointFlags

Test-point participation flags. Each bool selects one of the four

## Signature

```rust
pub struct TestpointFlags
```

## Decorators

- `derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Test-point participation flags. Each bool selects one of the four
test-point columns in Altium's Testpoint sub-section
(Top Assembly / Top Fab / Bottom Assembly / Bottom Fab).
[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `top_assembly`
- `top_fab`
- `bottom_assembly`
- `bottom_fab`

## Source
Lines 141–150 in `crates/oxide-sketch/src/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-sketch/src/attr.md) |
