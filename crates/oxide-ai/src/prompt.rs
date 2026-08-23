//! AI prompt templates and JSON schema builders for LLM integration.

/// Generate a strict JSON schema prompt instructing the LLM to output a valid `CircuitIntent`.
pub fn generate_circuit_generation_prompt(user_prompt: &str) -> String {
    format!(
        r#"You are Oxide EDA's AI Circuit Synthesis Engine.
Your goal is to parse user electronic requirements into a structured, validated circuit graph.

User Request: "{user_prompt}"

Output MUST be a single, valid JSON object strictly matching this schema:
{{
  "prompt": "{user_prompt}",
  "components": [
    {{
      "function": "<Exact Component Name or Function, e.g. ESP32-S3, TP4056, USB-C 16-Pin>",
      "designator_prefix": "<U, C, R, J, D>"
    }}
  ],
  "connections": [
    {{
      "net_name": "<NET_NAME, e.g. VBUS, GND, USB_D+, USB_D->",
      "from": ["<Source Component Function>", "<Pin Name>"],
      "to": ["<Destination Component Function>", "<Pin Name>"]
    }}
  ],
  "constraints": [
    "<Engineering constraints, e.g. 90 Ohm differential pair for USB>"
  ]
}}

Rules:
1. Always include necessary power pins (VBUS, +3V3, GND).
2. For microcontrollers, include power decoupling capacitors and boot/reset pullups.
3. For USB data lines, specify 90 Ohm differential pair constraint.
4. Output JSON ONLY, no surrounding markdown fences or commentary."#
    )
}
