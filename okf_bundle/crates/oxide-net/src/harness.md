---
okf_version: "0.2"
type: Module
title: harness
description: Structured Signal Harness Architecture.
resource: crates/oxide-net/src/harness.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:32:38Z"
concept_id: crates/oxide-net/src/harness
language: rust
---

# harness

Structured Signal Harness Architecture.

## Docstring

Structured Signal Harness Architecture.

Conforms to Master Technical Directive §5.2:
Heterogeneous Signal Packaging: Signal harnesses combine differential pairs, buses,
and single-ended control nets into a unified, strongly-typed logical connection bundle.

## Relationships

| Type | Target |
|------|--------|
| related | [HarnessElement](/crates/oxide-net/src/harness/HarnessElement.md) |
| related | [SignalHarness](/crates/oxide-net/src/harness/SignalHarness.md) |
| related | [new](/crates/oxide-net/src/harness/new.md) |
| related | [add_net](/crates/oxide-net/src/harness/add_net.md) |
| related | [add_bus](/crates/oxide-net/src/harness/add_bus.md) |
| related | [add_diff_pair](/crates/oxide-net/src/harness/add_diff_pair.md) |
| related | [expand_nets](/crates/oxide-net/src/harness/expand_nets.md) |
| related | [validate_compatibility](/crates/oxide-net/src/harness/validate_compatibility.md) |
| related | [new](/crates/oxide-net/src/harness/new.md) |
| related | [add_net](/crates/oxide-net/src/harness/add_net.md) |
| related | [add_bus](/crates/oxide-net/src/harness/add_bus.md) |
| related | [add_diff_pair](/crates/oxide-net/src/harness/add_diff_pair.md) |
| related | [expand_nets](/crates/oxide-net/src/harness/expand_nets.md) |
| related | [validate_compatibility](/crates/oxide-net/src/harness/validate_compatibility.md) |
| related | [test_signal_harness_expansion_and_validation](/crates/oxide-net/src/harness/test_signal_harness_expansion_and_validation.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
