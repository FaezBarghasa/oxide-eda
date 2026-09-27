---
okf_version: "0.2"
type: Function
title: content_erc
resource: crates/oxide-app/src/preferences/erc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/preferences/erc/content_erc
language: rust
---

# content_erc

## Signature

```rust
pub(super) fn content_erc(
    overrides: &'a std::collections::HashMap<oxide_erc::RuleKind, oxide_erc::Severity>,
) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Source
Lines 9–132 in `crates/oxide-app/src/preferences/erc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc](/crates/oxide-app/src/preferences/erc.md) |
| calls | [severity_label](/crates/oxide-app/src/preferences/erc/severity_label.md) |
| calls | [severity_bg](/crates/oxide-app/src/preferences/erc/severity_bg.md) |
| called_by | [build_content](/crates/oxide-app/src/preferences/mod/build_content.md) |
