---
okf_version: "0.2"
type: Function
title: claim_first_report
description: "True the first time `dir` fails this session."
resource: crates/oxide-app/src/app/dir_listing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dir_listing/claim_first_report
language: rust
---

# claim_first_report

True the first time `dir` fails this session.

## Signature

```rust
fn claim_first_report(dir: &Path) -> bool
```

## Docstring

True the first time `dir` fails this session.

The hot callers rebuild a whole panel context: `refresh_panel_ctx`
alone has 169 call sites, 41 of them in the pad editor, and the
footprint context is rebuilt on the same cadence. Reporting per call
would push the user's ERC results out of the 200-entry Messages ring
within a few keystrokes, so a given directory is reported once and
the repeats are suppressed. A listing failure of this kind is a
standing condition, not an event — one record is the whole story.

## Source
Lines 72–81 in `crates/oxide-app/src/app/dir_listing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dir_listing](/crates/oxide-app/src/app/dir_listing.md) |
| called_by | [list_dir_or_report](/crates/oxide-app/src/app/dir_listing/list_dir_or_report.md) |
