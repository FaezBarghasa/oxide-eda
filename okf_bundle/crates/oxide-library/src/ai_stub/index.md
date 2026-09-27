# ai_stub

## Classs

- [PinGuess](PinGuess.md) — One pin guessed from a datasheet table row.
- [PinoutGuess](PinoutGuess.md) — Result of the heuristic extractor.

## Functions

- [classifies_power_input_output_passive](classifies_power_input_output_passive.md) — [test]
- [classify_pin_kind](classify_pin_kind.md) — Classify a pin by name + description keywords.
- [dedupes_repeated_pin_numbers](dedupes_repeated_pin_numbers.md) — [test]
- [empty_text_yields_zero_confidence](empty_text_yields_zero_confidence.md) — [test]
- [extract_pinout](extract_pinout.md) — Heuristic pinout extractor. **No LLM**, **no network**.
- [extract_pinout_from_text](extract_pinout_from_text.md) — Pure-text variant — split out for unit-testing without a real PDF.
- [garbage_text_yields_low_confidence](garbage_text_yields_low_confidence.md) — [test]
- [parse_pin_row](parse_pin_row.md) — Match a single pin-table row.
- [parses_lm317_style_table_from_plain_text](parses_lm317_style_table_from_plain_text.md) — [test]
- [pin_row_regex](pin_row_regex.md) — Lazily compiled regex matching a pin-table row.
- [pinout_guess_round_trips_via_serde](pinout_guess_round_trips_via_serde.md) — [test]
- [rejects_long_garbage_token_as_pin_name](rejects_long_garbage_token_as_pin_name.md) — [test]
- [score_confidence](score_confidence.md) — Compute confidence in `[0, 1]` from the parsed pin list and the source
