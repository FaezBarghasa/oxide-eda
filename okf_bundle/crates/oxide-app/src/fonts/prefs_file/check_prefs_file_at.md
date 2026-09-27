---
okf_version: "0.2"
type: Function
title: check_prefs_file_at
description: "Is the `prefs.json` at `path` in a state a write can build on?"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/check_prefs_file_at
language: rust
---

# check_prefs_file_at

Is the `prefs.json` at `path` in a state a write can build on?

## Signature

```rust
pub fn check_prefs_file_at(path: &Path) -> Result<(), PrefsLoadError>
```

## Visibility

- `pub`

## Docstring

Is the `prefs.json` at `path` in a state a write can build on?

Pure and read-only — no logging, no latch, no file creation. It is
[`load_for_update`] with the loaded map thrown away, deliberately:
one source of truth means the banner can never disagree with what the
next [`update_prefs_json`] will actually do. `Ok(())` therefore covers
genuine absence and an all-whitespace file as well as a valid object,
because those are exactly the inputs a write starts fresh from.

It must NOT touch the refusal latch. The boot report and the
write-refusal report say different things — one "could not be
loaded", one "were not saved" — and both are true; keeping them
independent keeps a probe from suppressing a refusal the user has not
been shown yet.

## Source
Lines 221–223 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [load_for_update](/crates/oxide-app/src/fonts/prefs_file/load_for_update.md) |
| called_by | [check_prefs_file](/crates/oxide-app/src/fonts/prefs_file/check_prefs_file.md) |
| called_by | [checking_a_file_never_creates_or_modifies_it](/crates/oxide-app/src/fonts/prefs_file/checking_a_file_never_creates_or_modifies_it.md) |
| called_by | [checking_a_malformed_file_reports_a_parse_error](/crates/oxide-app/src/fonts/prefs_file/checking_a_malformed_file_reports_a_parse_error.md) |
| called_by | [checking_a_non_object_root_reports_it](/crates/oxide-app/src/fonts/prefs_file/checking_a_non_object_root_reports_it.md) |
| called_by | [checking_a_valid_object_reports_it_as_healthy](/crates/oxide-app/src/fonts/prefs_file/checking_a_valid_object_reports_it_as_healthy.md) |
| called_by | [checking_an_absent_file_reports_it_as_healthy](/crates/oxide-app/src/fonts/prefs_file/checking_an_absent_file_reports_it_as_healthy.md) |
| called_by | [checking_an_empty_file_reports_it_as_healthy](/crates/oxide-app/src/fonts/prefs_file/checking_an_empty_file_reports_it_as_healthy.md) |
