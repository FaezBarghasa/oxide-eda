---
okf_version: "0.2"
type: Function
title: prefs_path_lives_under_the_shared_config_root
description: "`prefs_path()` is private (unlike the other three prefs"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/prefs_path_lives_under_the_shared_config_root
language: rust
---

# prefs_path_lives_under_the_shared_config_root

`prefs_path()` is private (unlike the other three prefs

## Signature

```rust
fn prefs_path_lives_under_the_shared_config_root()
```

## Decorators

- `test`

## Docstring

`prefs_path()` is private (unlike the other three prefs
resolvers, which are all `pub`), so it can't be reached from an
integration test in `tests/` — this is that resolver's half of
the #440 guard. Proves fonts' prefs file lands under the same
`config_root()` the other three share, under the test redirect.
[test]

## Source
Lines 874–881 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [config_root](/crates/oxide-app/src/config_root/config_root.md) |
