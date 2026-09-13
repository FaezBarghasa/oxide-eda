use oxide_ai::{
    CircuitIntent, ComponentRequest, Distributor, StandardCatalogService, ValidationError,
    generate_circuit_generation_prompt, validate_and_enrich,
};

#[test]
fn test_json_intent_serialization() {
    let mut intent =
        CircuitIntent::new("Create an ESP32-S3 circuit with USB-C and a TP4056 LiPo charger");
    intent.add_component(ComponentRequest::new("ESP32-S3 Microcontroller", "U"));
    intent.add_component(ComponentRequest::new("TP4056 LiPo Charger", "U"));
    intent.add_component(ComponentRequest::new("USB-C Receptacle", "J"));

    intent.add_connection(
        "VBUS",
        ("USB-C Receptacle", "VBUS"),
        ("TP4056 LiPo Charger", "VIN"),
    );
    intent.add_connection(
        "GND",
        ("USB-C Receptacle", "GND"),
        ("TP4056 LiPo Charger", "GND"),
    );

    // Serialize to JSON and parse back
    let json_str = serde_json::to_string_pretty(&intent).expect("Serialization failed");
    let parsed: CircuitIntent = serde_json::from_str(&json_str).expect("Deserialization failed");

    assert_eq!(parsed.components.len(), 3);
    assert_eq!(parsed.connections.len(), 2);
    assert_eq!(parsed.connections[0].net_name, "VBUS");
}

#[test]
fn test_real_part_validator_enrichment() {
    let mut intent = CircuitIntent::new("Minimal ESP32 with USB-C and BMP280");
    intent.add_component(ComponentRequest::new("ESP32-S3", "U"));
    intent.add_component(ComponentRequest::new("BMP280 Sensor", "U"));
    intent.add_component(ComponentRequest::new("USB-C Connector", "J"));

    let catalog = StandardCatalogService::new();
    let result = validate_and_enrich(&mut intent, &catalog);
    assert!(result.is_ok(), "Validation should succeed for known parts");

    assert_eq!(
        intent.components[0].verified_part_number.as_deref(),
        Some("ESP32-S3-WROOM-1-N8R8")
    );
    assert_eq!(
        intent.components[0].preferred_distributor,
        Some(Distributor::Lcsc)
    );
    assert!(intent.components[0].in_stock_quantity.unwrap() > 0);

    assert_eq!(
        intent.components[1].verified_part_number.as_deref(),
        Some("BMP280")
    );
    assert_eq!(
        intent.components[1].preferred_distributor,
        Some(Distributor::DigiKey)
    );

    assert!(intent.is_validated);
}

#[test]
fn test_unmatched_hallucinated_part_fails_validation() {
    let mut intent = CircuitIntent::new("Circuit with non-existent fantasy part");
    intent.add_component(ComponentRequest::new("Quantum_Flux_Super_IC_9999", "U"));

    let catalog = StandardCatalogService::new();
    let result = validate_and_enrich(&mut intent, &catalog);

    assert!(matches!(result, Err(ValidationError::UnmatchedPart { .. })));
    assert!(!intent.is_validated);
}

#[test]
fn test_prompt_generation() {
    let prompt = generate_circuit_generation_prompt("ESP32-S3 USB circuit");
    assert!(prompt.contains("User Request: \"ESP32-S3 USB circuit\""));
    assert!(prompt.contains("Output MUST be a single, valid JSON object"));
}
