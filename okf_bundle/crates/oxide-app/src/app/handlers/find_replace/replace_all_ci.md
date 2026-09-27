---
okf_version: "0.2"
type: Function
title: replace_all_ci
description: "Replace every case-insensitive occurrence of `needle` in `haystack`"
resource: crates/oxide-app/src/app/handlers/find_replace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/find_replace/replace_all_ci
language: rust
---

# replace_all_ci

Replace every case-insensitive occurrence of `needle` in `haystack`

## Signature

```rust
fn replace_all_ci(haystack: &str, needle: &str, replacement: &str) -> String
```

## Docstring

Replace every case-insensitive occurrence of `needle` in `haystack`
with `replacement`, preserving the rest of the string. Char-based so
multi-byte content keeps valid boundaries; ASCII case-folding matches
the `contains` used to find the hit. An empty needle is a no-op (the
find layer never produces an empty query).

## Source
Lines 185–208 in `crates/oxide-app/src/app/handlers/find_replace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [find_replace](/crates/oxide-app/src/app/handlers/find_replace.md) |
| called_by | [handle_find_replace_message](/crates/oxide-app/src/app/handlers/find_replace/handle_find_replace_message.md) |
