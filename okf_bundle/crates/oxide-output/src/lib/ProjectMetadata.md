---
okf_version: "0.2"
type: Class
title: ProjectMetadata
description: "Title-block / project-file metadata used to resolve `${TITLE}`, `${REV}`,"
resource: crates/oxide-output/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:20:47Z"
concept_id: crates/oxide-output/src/lib/ProjectMetadata
language: rust
---

# ProjectMetadata

Title-block / project-file metadata used to resolve `${TITLE}`, `${REV}`,

## Signature

```rust
pub struct ProjectMetadata
```

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

Title-block / project-file metadata used to resolve `${TITLE}`, `${REV}`,
etc. and to stamp the exported artifact.
[derive(Debug, Clone, Default)]

## Methods

- `title`
- `revision`
- `date`
- `company`
- `comments`
- `custom_fields`

## Source
Lines 95–102 in `crates/oxide-output/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-output/src/lib.md) |
