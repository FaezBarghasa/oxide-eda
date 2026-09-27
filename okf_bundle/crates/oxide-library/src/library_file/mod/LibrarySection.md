---
okf_version: "0.2"
type: Class
title: LibrarySection
description: "`[library]` block — human-readable name + description. The `library_id`"
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod/LibrarySection
language: rust
---

# LibrarySection

`[library]` block — human-readable name + description. The `library_id`

## Signature

```rust
pub struct LibrarySection
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

`[library]` block — human-readable name + description. The `library_id`
lives at the TOML root, not here.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `name`
- `description`

## Source
Lines 93–97 in `crates/oxide-library/src/library_file/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_file](/crates/oxide-library/src/library_file/mod.md) |
