# digikey_oauth

## Classs

- [CancelHandle](CancelHandle.md) — Hands the caller a way to cancel the in-flight handshake. Cloning
- [FailingReader](FailingReader.md) — A socket that refuses every read, standing in for a connection
- [Outcome](Outcome.md) — Outcome of the OAuth handshake. Returned via the iced `Task` that

## Functions

- [an_empty_request_line_is_still_a_malformed_redirect](an_empty_request_line_is_still_a_malformed_redirect.md) — An empty first line is what a failed read used to look like. It
- [bind_callback_listener](bind_callback_listener.md) — Bind a localhost port for the OAuth callback server. Returns the
- [callback_params](callback_params.md) — Turn what came off the callback socket into the `(code, state)` pair,
- [callback_params_returns_code_and_state_on_a_good_redirect](callback_params_returns_code_and_state_on_a_good_redirect.md) — [test]
- [callback_query_keys](callback_query_keys.md) — Names of the query parameters on a callback request line, joined for
- [callback_query_keys_logs_names_without_values](callback_query_keys_logs_names_without_values.md) — [test]
- [cancel](cancel.md) — Mark the flow as cancelled. The owning thread mutates the
- [cancel](cancel_1.md) — Mark the flow as cancelled. The owning thread mutates the
- [cancel_handle_observable_across_clones](cancel_handle_observable_across_clones.md) — [test]
- [default](default.md)
- [default](default_1.md)
- [failed_reason](failed_reason.md)
- [failure_from](failure_from.md)
- [from_flag](from_flag.md) — Wrap an existing flag — used by the dispatcher so the UI can
- [from_flag](from_flag_1.md) — Wrap an existing flag — used by the dispatcher so the UI can
- [hex_digit](hex_digit.md)
- [is_cancelled](is_cancelled.md)
- [is_cancelled](is_cancelled_1.md)
- [missing_client_id_fails_clearly](missing_client_id_fails_clearly.md) — [test]
- [new](new.md)
- [new](new_1.md)
- [parse_callback](parse_callback.md) — Extract `code` and `state` query params from the first request
- [parse_callback_extracts_code_and_state](parse_callback_extracts_code_and_state.md) — [test]
- [parse_callback_handles_url_encoding](parse_callback_handles_url_encoding.md) — [test]
- [parse_callback_returns_none_on_missing_query](parse_callback_returns_none_on_missing_query.md) — [test]
- [read](read.md)
- [read](read_1.md)
- [read_env_credentials](read_env_credentials.md) — Read environment-supplied DigiKey credentials. Returns empty
- [read_first_line](read_first_line.md) — Read the first line of the HTTP request — only the line we need
- [read_first_line_propagates_a_socket_read_error](read_first_line_propagates_a_socket_read_error.md) — [test]
- [read_first_line_returns_the_request_line](read_first_line_returns_the_request_line.md) — [test]
- [run_blocking](run_blocking.md) — Body of the synchronous handshake. Runs on a worker thread (caller
- [socket_read_error_and_malformed_redirect_report_different_reasons](socket_read_error_and_malformed_redirect_report_different_reasons.md) — The row this test pins: a socket that could not be read and a
- [url_decode](url_decode.md)
