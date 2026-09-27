---
okf_version: "0.2"
type: Function
title: exchange_code
description: "Step 2: exchange the redirected `code` for tokens; persist the"
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/exchange_code_1
language: rust
---

# exchange_code

Step 2: exchange the redirected `code` for tokens; persist the

## Signature

```rust
pub fn exchange_code(
        &self,
        code: &str,
        verifier: PkceCodeVerifier,
        returned_state: &str,
        expected_state: &CsrfToken,
    ) -> Result<String, DigiKeyAuthError>
```

## Visibility

- `pub`

## Docstring

Step 2: exchange the redirected `code` for tokens; persist the
refresh token in keyring and return the access token.

H3: this call now requires the CSRF state token returned by the
authorization redirect (`returned_state`) to match the one issued by
[`Self::start_authorization`] (`expected_state`). Without this check,
an attacker who can forge a redirect (e.g. via DNS rebinding or a
malicious deep-link) could complete the OAuth exchange with an
attacker-controlled `code` and capture the resulting refresh token
into the victim's keyring.

We compare with `str::eq`. Constant-time comparison would be cleaner
in principle, but DigiKey's CSRF tokens are 128+ bits of entropy and
the exchange is a one-shot per browser session — the timing
side-channel surface is bounded to a single guess per OAuth flow,
well below the threshold where ConstantTime matters. Document this
inline so future audits don't flag it.

## Source
Lines 184–209 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |
| calls | [build_oauth_client](/crates/oxide-library/src/distributors/digikey/build_oauth_client.md) |
