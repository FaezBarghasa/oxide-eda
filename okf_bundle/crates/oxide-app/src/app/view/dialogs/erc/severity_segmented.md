---
okf_version: "0.2"
type: Function
title: severity_segmented
resource: crates/oxide-app/src/app/view/dialogs/erc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/erc/severity_segmented
language: rust
---

# severity_segmented

## Signature

```rust
fn severity_segmented(
    rule: oxide_erc::RuleKind,
    current: oxide_erc::Severity,
    border: Color,
    text_muted: Color,
) -> Element<'static, Message>
```

## Source
Lines 175–230 in `crates/oxide-app/src/app/view/dialogs/erc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc](/crates/oxide-app/src/app/view/dialogs/erc.md) |
