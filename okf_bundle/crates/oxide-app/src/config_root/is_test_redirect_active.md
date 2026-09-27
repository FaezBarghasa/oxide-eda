---
okf_version: "0.2"
type: Function
title: is_test_redirect_active
description: "True when the test/dev redirect gate is active for this build:"
resource: crates/oxide-app/src/config_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/config_root/is_test_redirect_active
language: rust
---

# is_test_redirect_active

True when the test/dev redirect gate is active for this build:

## Signature

```rust
pub(crate) fn is_test_redirect_active() -> bool
```

## Visibility

- `pub(crate)`

## Docstring

True when the test/dev redirect gate is active for this build:
oxide-app's own unit-test build (`cfg(test)`) or the
`test-prefs-redirect` feature (activated for every crate that links
oxide-app as a dev-dependency — see `Cargo.toml`). Written once here
so no call site re-derives the predicate. [`config_root`] uses it
directly; [`crate::fonts`] also uses it to decide whether its
one-shot legacy-prefs migration should run at all — that migration
must not touch the developer's real config directory during a test
run (issue #437).

## Source
Lines 36–38 in `crates/oxide-app/src/config_root.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config_root](/crates/oxide-app/src/config_root.md) |
| called_by | [config_root](/crates/oxide-app/src/config_root/config_root.md) |
| called_by | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
