---
okf_version: "0.2"
type: Function
title: template_with_required_and_optional_fully_populated
description: Fully-populated edit case for a class that has both required and
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/template_with_required_and_optional_fully_populated
language: rust
---

# template_with_required_and_optional_fully_populated

Fully-populated edit case for a class that has both required and

## Signature

```rust
fn template_with_required_and_optional_fully_populated()
```

## Decorators

- `test`

## Docstring

Fully-populated edit case for a class that has both required and
optional slots. With every required slot filled the validator
must report no missing-required violations; optional slots may
stay absent without raising any error.
[test]

## Source
Lines 561–588 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
| calls | [missing_required_for_test](/crates/oxide-app/src/library/editor/params/missing_required_for_test.md) |
