---
okf_version: "0.2"
type: Function
title: build_prefs_file_banner
description: "Dialog-wide alert strip for a `prefs.json` that could not be loaded,"
resource: crates/oxide-app/src/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/preferences/mod/build_prefs_file_banner
language: rust
---

# build_prefs_file_banner

Dialog-wide alert strip for a `prefs.json` that could not be loaded,

## Signature

```rust
fn build_prefs_file_banner(
    load_error: Option<&'a str>,
    status: &'a str,
) -> Option<Element<'a, PrefMsg>>
```

## Type Parameters

- `'a`

## Docstring

Dialog-wide alert strip for a `prefs.json` that could not be loaded,
carrying the result of the recovery action once it has run (#602).

`None` — and therefore a dialog byte-for-byte what it is today — is
the healthy case: nothing to report and nothing to offer. That is the
acceptance criterion "with a healthy `prefs.json`, nothing about the
current behaviour changes".

Same alert-strip shape as [`build_footer`]: a `background.weak` fill
with a `background.strong` hairline, every colour resolved from the
theme rather than hardcoded, so it reads on light and dark palettes.
The button is [`secondary_button_style`] and not `danger_button_style`
on purpose — the action is neutral. Nothing is deleted; the file the
user may still want is renamed, not removed.

## Source
Lines 742–835 in `crates/oxide-app/src/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/preferences/mod.md) |
| calls | [prefs_file_path](/crates/oxide-app/src/fonts/prefs_file/prefs_file_path.md) |
| called_by | [build_dialog](/crates/oxide-app/src/preferences/mod/build_dialog.md) |
