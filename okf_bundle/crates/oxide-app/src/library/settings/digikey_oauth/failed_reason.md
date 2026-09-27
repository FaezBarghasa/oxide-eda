---
okf_version: "0.2"
type: Function
title: failed_reason
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/failed_reason
language: rust
---

# failed_reason

## Signature

```rust
fn failed_reason(outcome: Outcome) -> String
```

## Source
Lines 444–449 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| called_by | [an_empty_request_line_is_still_a_malformed_redirect](/crates/oxide-app/src/library/settings/digikey_oauth/an_empty_request_line_is_still_a_malformed_redirect.md) |
| called_by | [socket_read_error_and_malformed_redirect_report_different_reasons](/crates/oxide-app/src/library/settings/digikey_oauth/socket_read_error_and_malformed_redirect_report_different_reasons.md) |
