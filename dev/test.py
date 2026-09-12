from hbp100 import HBP100

DEFAULTS = [
    {
        "name": "Name",
        "entity_type": "NAME",
        "pattern": r"\b[A-Z][a-z]+ [A-Z][a-z]+\b",
        "confidence": 0.90,
    },
    {
        "name": "MRN",
        "entity_type": "ID",
        "pattern": r"\b\d{6}\b",
        "confidence": 0.95,
    },
    {
        "name": "Phone",
        "entity_type": "PHONE",
        "pattern": r"\b\d{10}\b",
        "confidence": 0.90,
    },
    {
        "name": "Email",
        "entity_type": "EMAIL",
        "pattern": r"\b[\w.+-]+@[\w-]+\.[\w.-]+\b",
        "confidence": 0.95,
    },
    {
        "name": "Address",
        "entity_type": "ADDRESS",
        "pattern": r"\b\d+\s+[A-Z][a-z]+\s+(St|Ave|Rd|Blvd)\b",
        "confidence": 0.85,
    },
]


def main():
    engine = HBP100()

    import json
    for cfg in DEFAULTS:
        engine.add_extractor(json.dumps(cfg))

    test_texts = [
        "Patient John Doe, MRN: 123456, Phone: 9876543210",
        "The patient was diagnosed with diabetes",
        "Contact: john.doe@email.com, Address: 123 Main St",
    ]

    for text in test_texts:
        print(f"\n Input: {text}")
        result = engine.process(text)
        print(f"Masked: {result['masked_text']}")
        print(f"Entities: {len(result['entities'])}")
        print(f"Has PII: {result['has_pii']}")

        for decision in result["decisions"][:3]:
            print(
                f"  {decision['entity_type']} -> "
                f"{decision['decision']} ({decision['confidence']})"
            )


if __name__ == "__main__":
    main()