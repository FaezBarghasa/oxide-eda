---
okf_version: "0.2"
type: Module
title: sim
description: "`SimModel` primitive — SPICE / Verilog-A model body, reusable across MPNs."
resource: crates/oxide-library/src/primitive/sim.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:11:54Z"
concept_id: crates/oxide-library/src/primitive/sim
language: rust
---

# sim

`SimModel` primitive — SPICE / Verilog-A model body, reusable across MPNs.

## Docstring

`SimModel` primitive — SPICE / Verilog-A model body, reusable across MPNs.

Per `v0.9-refactor-2-plan.md` §2.3, a `SimModel` carries the model
body and a default symbol-pin → SPICE-node mapping. A binding `Component`
references it via `Revision::sim_ref` and may override the node map for
a specific MPN.

## Relationships

| Type | Target |
|------|--------|
| related | [SimKind](/crates/oxide-library/src/primitive/sim/SimKind.md) |
| related | [SimModel](/crates/oxide-library/src/primitive/sim/SimModel.md) |
| related | [default_sim_version](/crates/oxide-library/src/primitive/sim/default_sim_version.md) |
| related | [empty](/crates/oxide-library/src/primitive/sim/empty.md) |
| related | [empty](/crates/oxide-library/src/primitive/sim/empty.md) |
| related | [SimFile](/crates/oxide-library/src/primitive/sim/SimFile.md) |
| related | [default_sim_format](/crates/oxide-library/src/primitive/sim/default_sim_format.md) |
| related | [SimFileWire](/crates/oxide-library/src/primitive/sim/SimFileWire.md) |
| related | [SimModelWire](/crates/oxide-library/src/primitive/sim/SimModelWire.md) |
| related | [from_model](/crates/oxide-library/src/primitive/sim/from_model.md) |
| related | [from_bytes](/crates/oxide-library/src/primitive/sim/from_bytes.md) |
| related | [from_toml_str](/crates/oxide-library/src/primitive/sim/from_toml_str.md) |
| related | [to_toml_string](/crates/oxide-library/src/primitive/sim/to_toml_string.md) |
| related | [get_model](/crates/oxide-library/src/primitive/sim/get_model.md) |
| related | [from_model](/crates/oxide-library/src/primitive/sim/from_model.md) |
| related | [from_bytes](/crates/oxide-library/src/primitive/sim/from_bytes.md) |
| related | [from_toml_str](/crates/oxide-library/src/primitive/sim/from_toml_str.md) |
| related | [to_toml_string](/crates/oxide-library/src/primitive/sim/to_toml_string.md) |
| related | [get_model](/crates/oxide-library/src/primitive/sim/get_model.md) |
| related | [parse_pspice_library](/crates/oxide-library/src/primitive/sim/parse_pspice_library.md) |
| related | [SimFileError](/crates/oxide-library/src/primitive/sim/SimFileError.md) |
| related | [sim_model_round_trip](/crates/oxide-library/src/primitive/sim/sim_model_round_trip.md) |
| related | [sim_kind_round_trip_all_variants](/crates/oxide-library/src/primitive/sim/sim_kind_round_trip_all_variants.md) |
| related | [parse_pspice_library_subckt_and_model](/crates/oxide-library/src/primitive/sim/parse_pspice_library_subckt_and_model.md) |
| related | [empty_sim_model_carries_no_body](/crates/oxide-library/src/primitive/sim/empty_sim_model_carries_no_body.md) |
| related | [sim_file_from_model_wraps_into_one_element_vec](/crates/oxide-library/src/primitive/sim/sim_file_from_model_wraps_into_one_element_vec.md) |
| related | [sim_file_toml_round_trip_empty_body](/crates/oxide-library/src/primitive/sim/sim_file_toml_round_trip_empty_body.md) |
| related | [sim_file_toml_round_trip_with_full_body](/crates/oxide-library/src/primitive/sim/sim_file_toml_round_trip_with_full_body.md) |
| related | [sim_file_to_toml_emits_body_as_literal_multiline](/crates/oxide-library/src/primitive/sim/sim_file_to_toml_emits_body_as_literal_multiline.md) |
| related | [sim_file_from_bytes_decodes_toml_envelope](/crates/oxide-library/src/primitive/sim/sim_file_from_bytes_decodes_toml_envelope.md) |
| related | [sim_file_from_bytes_rejects_empty_payload](/crates/oxide-library/src/primitive/sim/sim_file_from_bytes_rejects_empty_payload.md) |
| related | [sim_file_unsupported_format_token_is_rejected](/crates/oxide-library/src/primitive/sim/sim_file_unsupported_format_token_is_rejected.md) |
| related | [sim_file_to_toml_rejects_triple_quote_in_body](/crates/oxide-library/src/primitive/sim/sim_file_to_toml_rejects_triple_quote_in_body.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
