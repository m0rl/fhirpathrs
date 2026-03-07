import pytest
import fhirpathrs


def test_simple_field():
    assert fhirpathrs.evaluate("name", {"name": "John"}) == "John"


def test_nested_path():
    data = {"a": {"b": {"c": 42}}}
    assert fhirpathrs.evaluate("a.b.c", data) == 42


def test_collection_navigation():
    data = {"name": [{"given": ["John", "James"], "family": "Doe"}]}
    result = fhirpathrs.evaluate("name.given", data)
    assert result == ["John", "James"]


def test_where_filter():
    data = {
        "entry": [
            {"value": 10, "active": True},
            {"value": 20, "active": False},
            {"value": 30, "active": True},
        ]
    }
    result = fhirpathrs.evaluate("entry.where(active = true).value", data)
    assert result == [10, 30]


def test_arithmetic():
    assert fhirpathrs.evaluate("1 + 2 * 3") == 7


def test_boolean_logic():
    assert fhirpathrs.evaluate("true and false") is False
    assert fhirpathrs.evaluate("true or false") is True


def test_external_constants():
    result = fhirpathrs.evaluate(
        "%threshold + 10", constants={"threshold": 100}
    )
    assert result == 110


def test_string_functions():
    result = fhirpathrs.evaluate("name.length()", {"name": "hello"})
    assert result == 5


def test_count():
    data = {"items": [1, 2, 3, 4, 5]}
    assert fhirpathrs.evaluate("items.count()", data) == 5


def test_exists():
    data = {"name": "John"}
    assert fhirpathrs.evaluate("name.exists()", data) is True
    assert fhirpathrs.evaluate("missing.exists()", data) is False


def test_parse_error():
    with pytest.raises(ValueError, match="Parse error"):
        fhirpathrs.evaluate("!@#invalid")


def test_empty_collection():
    data = {"items": []}
    assert fhirpathrs.evaluate("items", data) == []


def test_null_data():
    assert fhirpathrs.evaluate("name", {"name": None}) is None


def test_fhir_resource_type():
    data = {
        "resourceType": "Patient",
        "id": "123",
        "active": True,
        "name": [{"given": ["Jane"], "family": "Doe"}],
    }
    assert fhirpathrs.evaluate("active", data) is True
    assert fhirpathrs.evaluate("name.family", data) == ["Doe"]
    assert fhirpathrs.evaluate("name.given", data) == ["Jane"]


def test_percent_resolves_from_data():
    qr = {
        "resourceType": "QuestionnaireResponse",
        "status": "completed",
        "item": [
            {
                "linkId": "blood-pressure",
                "item": [
                    {"linkId": "systolic", "answer": [{"valueInteger": 120}]},
                    {"linkId": "diastolic", "answer": [{"valueInteger": 80}]},
                ],
            },
            {
                "linkId": "heart-rate",
                "answer": [{"valueInteger": 72}],
            },
        ],
    }
    data = {"QuestionnaireResponse": qr}
    result = fhirpathrs.evaluate(
        "%QuestionnaireResponse.item.where(linkId = 'heart-rate').answer.valueInteger",
        data,
    )
    assert result == [72]


def test_percent_resolves_from_constants():
    qr = {
        "resourceType": "QuestionnaireResponse",
        "item": [
            {"linkId": "q1", "answer": [{"valueString": "yes"}]},
        ],
    }
    result = fhirpathrs.evaluate(
        "%QuestionnaireResponse.item.where(linkId = 'q1').answer.valueString",
        constants={"QuestionnaireResponse": qr},
    )
    assert result == ["yes"]


def test_percent_constants_take_priority_over_data():
    result = fhirpathrs.evaluate(
        "%x", {"x": "from-data"}, constants={"x": "from-constants"}
    )
    assert result == "from-constants"


def test_predefined_ucum():
    assert fhirpathrs.evaluate("%ucum") == "http://unitsofmeasure.org"


def test_predefined_sct():
    assert fhirpathrs.evaluate("%sct") == "http://snomed.info/sct"


def test_predefined_loinc():
    assert fhirpathrs.evaluate("%loinc") == "http://loinc.org"


def test_predefined_context():
    data = {"resourceType": "Patient", "id": "pat-1"}
    result = fhirpathrs.evaluate("%context.id", data)
    assert result == "pat-1"


def test_predefined_resource():
    data = {"resourceType": "Observation", "status": "final"}
    result = fhirpathrs.evaluate("%resource.status", data)
    assert result == "final"


def test_predefined_root_resource():
    data = {"resourceType": "Bundle", "type": "collection"}
    result = fhirpathrs.evaluate("%rootResource.type", data)
    assert result == "collection"


def test_predefined_can_be_overridden_by_constants():
    result = fhirpathrs.evaluate(
        "%ucum", constants={"ucum": "custom-ucum"}
    )
    assert result == "custom-ucum"
