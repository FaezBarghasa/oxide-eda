---
okf_version: "0.2"
type: Function
title: settle_deny
description: "Poll with a real probe write instead of trusting `icacls`'s exit"
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/settle_deny_1
language: rust
---

# settle_deny

Poll with a real probe write instead of trusting `icacls`'s exit

## Signature

```rust
fn settle_deny(dir: &std::path::Path)
```

## Decorators

- `cfg(windows)`

## Docstring

Poll with a real probe write instead of trusting `icacls`'s exit
code, so the caller never proceeds through a window where the
directory still silently accepts new files (#482).
[cfg(windows)]

## Source
Lines 524–536 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
