---
okf_version: "0.2"
type: Function
title: fixture_signature
description: Mint a signature for fixture commits without leaning on the
resource: crates/oxide-library/tests/project_file_history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/project_file_history/fixture_signature
language: rust
---

# fixture_signature

Mint a signature for fixture commits without leaning on the

## Signature

```rust
fn fixture_signature() -> git2::Signature<'static>
```

## Docstring

Mint a signature for fixture commits without leaning on the
caller's `git` config (CI machines often have neither set).

## Source
Lines 19–21 in `crates/oxide-library/tests/project_file_history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_file_history](/crates/oxide-library/tests/project_file_history.md) |
| called_by | [commit_file](/crates/oxide-library/tests/project_file_history/commit_file.md) |
