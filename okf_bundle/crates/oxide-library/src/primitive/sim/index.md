# sim

## Classs

- [SimFile](SimFile.md) — `.snxsim` container — Altium parity for SimModel storage. The
- [SimFileError](SimFileError.md) — Error variants raised by [`SimFile`] parsers + serialisers.
- [SimFileWire](SimFileWire.md) — [derive(Serialize, Deserialize)]
- [SimKind](SimKind.md) — SPICE / behavioural model dialect.
- [SimModel](SimModel.md) — Reusable simulation model. Bound by `Component::sim_ref`.
- [SimModelWire](SimModelWire.md) — [derive(Serialize, Deserialize)]

## Functions

- [default_sim_format](default_sim_format.md)
- [default_sim_version](default_sim_version.md)
- [empty](empty.md)
- [empty](empty_1.md)
- [empty_sim_model_carries_no_body](empty_sim_model_carries_no_body.md) — [test]
- [from_bytes](from_bytes.md) — Decode bytes as UTF-8 and parse via [`SimFile::from_toml_str`].
- [from_bytes](from_bytes_1.md) — Decode bytes as UTF-8 and parse via [`SimFile::from_toml_str`].
- [from_model](from_model.md) — Wrap a single `SimModel` into a one-element file envelope.
- [from_model](from_model_1.md) — Wrap a single `SimModel` into a one-element file envelope.
- [from_toml_str](from_toml_str.md) — Parse the TOML wire format. Format-token mismatch surfaces
- [from_toml_str](from_toml_str_1.md) — Parse the TOML wire format. Format-token mismatch surfaces
- [get_model](get_model.md) — Locate a model by UUID within this file.
- [get_model](get_model_1.md) — Locate a model by UUID within this file.
- [parse_pspice_library](parse_pspice_library.md) — Parse a PSpice library text payload containing one or more `.SUBCKT` blocks or `.MODEL` statements.
- [parse_pspice_library_subckt_and_model](parse_pspice_library_subckt_and_model.md) — [test]
- [sim_file_from_bytes_decodes_toml_envelope](sim_file_from_bytes_decodes_toml_envelope.md) — [test]
- [sim_file_from_bytes_rejects_empty_payload](sim_file_from_bytes_rejects_empty_payload.md) — [test]
- [sim_file_from_model_wraps_into_one_element_vec](sim_file_from_model_wraps_into_one_element_vec.md) — [test]
- [sim_file_to_toml_emits_body_as_literal_multiline](sim_file_to_toml_emits_body_as_literal_multiline.md) — [test]
- [sim_file_to_toml_rejects_triple_quote_in_body](sim_file_to_toml_rejects_triple_quote_in_body.md) — [test]
- [sim_file_toml_round_trip_empty_body](sim_file_toml_round_trip_empty_body.md) — [test]
- [sim_file_toml_round_trip_with_full_body](sim_file_toml_round_trip_with_full_body.md) — [test]
- [sim_file_unsupported_format_token_is_rejected](sim_file_unsupported_format_token_is_rejected.md) — [test]
- [sim_kind_round_trip_all_variants](sim_kind_round_trip_all_variants.md) — [test]
- [sim_model_round_trip](sim_model_round_trip.md) — [test]
- [to_toml_string](to_toml_string.md) — Serialise to canonical TOML. The per-model `body` field is
- [to_toml_string](to_toml_string_1.md) — Serialise to canonical TOML. The per-model `body` field is
