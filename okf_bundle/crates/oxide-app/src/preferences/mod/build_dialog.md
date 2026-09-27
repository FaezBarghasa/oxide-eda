---
okf_version: "0.2"
type: Function
title: build_dialog
resource: crates/oxide-app/src/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/preferences/mod/build_dialog
language: rust
---

# build_dialog

## Signature

```rust
fn build_dialog(v: PrefsView<'a>) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Source
Lines 365–479 in `crates/oxide-app/src/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/preferences/mod.md) |
| calls | [build_footer](/crates/oxide-app/src/preferences/mod/build_footer.md) |
| calls | [build_prefs_file_banner](/crates/oxide-app/src/preferences/mod/build_prefs_file_banner.md) |
| called_by | [view](/crates/oxide-app/src/preferences/mod/view.md) |
| called_by | [view_body](/crates/oxide-app/src/preferences/mod/view_body.md) |
