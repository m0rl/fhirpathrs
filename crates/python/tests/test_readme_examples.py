"""Tests for every Python code example in README.md."""

from fhirpathrs import evaluate


def test_quick_start_navigate_fields():
    patient = {
        "resourceType": "Patient",
        "id": "pat-42",
        "active": True,
        "name": [
            {"family": "Jetson", "given": ["George", "Astro"]},
            {"family": "Jetson-Spacely", "given": ["Elroy"]},
        ],
        "address": [{"city": "Orbit City", "state": "Space"}],
    }
    assert evaluate("name.family", patient) == ["Jetson", "Jetson-Spacely"]


def test_quick_start_filter_collections():
    patient = {
        "resourceType": "Patient",
        "id": "pat-42",
        "active": True,
        "name": [
            {"family": "Jetson", "given": ["George", "Astro"]},
            {"family": "Jetson-Spacely", "given": ["Elroy"]},
        ],
        "address": [{"city": "Orbit City", "state": "Space"}],
    }
    assert evaluate("name.where(family = 'Jetson').given", patient) == [
        "George",
        "Astro",
    ]


def test_quick_start_boolean_logic():
    patient = {
        "resourceType": "Patient",
        "id": "pat-42",
        "active": True,
        "name": [
            {"family": "Jetson", "given": ["George", "Astro"]},
            {"family": "Jetson-Spacely", "given": ["Elroy"]},
        ],
        "address": [{"city": "Orbit City", "state": "Space"}],
    }
    assert evaluate("name.exists() and active", patient) is True


def test_quick_start_count():
    patient = {
        "resourceType": "Patient",
        "id": "pat-42",
        "active": True,
        "name": [
            {"family": "Jetson", "given": ["George", "Astro"]},
            {"family": "Jetson-Spacely", "given": ["Elroy"]},
        ],
        "address": [{"city": "Orbit City", "state": "Space"}],
    }
    assert evaluate("name.count()", patient) == 2


def test_path_navigation():
    data = {"a": {"b": {"c": 42}}}
    assert evaluate("a.b.c", data) == 42


def test_where_filter():
    bundle = {
        "entry": [
            {
                "resource": {
                    "resourceType": "Observation",
                    "status": "final",
                    "value": 120,
                }
            },
            {"resource": {"resourceType": "Condition", "status": "active"}},
            {
                "resource": {
                    "resourceType": "Observation",
                    "status": "final",
                    "value": 80,
                }
            },
        ]
    }
    assert (
        evaluate(
            "entry.resource.where(resourceType = 'Observation').value",
            bundle,
        )
        == [120, 80]
    )


def test_questionnaire_response_systolic():
    data = {
        "QuestionnaireResponse": {
            "resourceType": "QuestionnaireResponse",
            "status": "completed",
            "item": [
                {
                    "linkId": "blood-pressure",
                    "item": [
                        {
                            "linkId": "systolic",
                            "answer": [{"valueInteger": 120}],
                        },
                        {
                            "linkId": "diastolic",
                            "answer": [{"valueInteger": 80}],
                        },
                    ],
                },
                {
                    "linkId": "heart-rate",
                    "answer": [{"valueInteger": 72}],
                },
            ],
        }
    }
    assert (
        evaluate(
            "%QuestionnaireResponse.repeat(item)"
            ".where(linkId = 'systolic').answer.valueInteger",
            data,
        )
        == [120]
    )


def test_questionnaire_response_heart_rate():
    data = {
        "QuestionnaireResponse": {
            "resourceType": "QuestionnaireResponse",
            "status": "completed",
            "item": [
                {
                    "linkId": "blood-pressure",
                    "item": [
                        {
                            "linkId": "systolic",
                            "answer": [{"valueInteger": 120}],
                        },
                        {
                            "linkId": "diastolic",
                            "answer": [{"valueInteger": 80}],
                        },
                    ],
                },
                {
                    "linkId": "heart-rate",
                    "answer": [{"valueInteger": 72}],
                },
            ],
        }
    }
    assert (
        evaluate(
            "%QuestionnaireResponse.repeat(item)"
            ".where(linkId = 'heart-rate').answer.valueInteger",
            data,
        )
        == [72]
    )


def test_questionnaire_response_all_answers():
    data = {
        "QuestionnaireResponse": {
            "resourceType": "QuestionnaireResponse",
            "status": "completed",
            "item": [
                {
                    "linkId": "blood-pressure",
                    "item": [
                        {
                            "linkId": "systolic",
                            "answer": [{"valueInteger": 120}],
                        },
                        {
                            "linkId": "diastolic",
                            "answer": [{"valueInteger": 80}],
                        },
                    ],
                },
                {
                    "linkId": "heart-rate",
                    "answer": [{"valueInteger": 72}],
                },
            ],
        }
    }
    result = evaluate(
        "%QuestionnaireResponse.repeat(item).answer.valueInteger", data
    )
    assert sorted(result) == [72, 80, 120]


def test_external_constants():
    data = {"items": [{"value": 10}, {"value": 50}, {"value": 90}]}
    assert (
        evaluate(
            "items.where(value > %threshold).count()",
            data,
            constants={"threshold": 25},
        )
        == 2
    )


def test_arithmetic():
    assert evaluate("1 + 2 * 3") == 7


def test_string_length():
    assert evaluate("name.length()", {"name": "hello"}) == 5


def test_string_upper():
    assert evaluate("name.upper()", {"name": "hello"}) == "HELLO"


def test_sum():
    data = {"scores": [10, 20, 30, 40, 50]}
    assert evaluate("scores.sum()", data) == 150


def test_avg():
    data = {"scores": [10, 20, 30, 40, 50]}
    assert evaluate("scores.avg()", data) == 30.0


def test_min():
    data = {"scores": [10, 20, 30, 40, 50]}
    assert evaluate("scores.min()", data) == 10


def test_max():
    data = {"scores": [10, 20, 30, 40, 50]}
    assert evaluate("scores.max()", data) == 50


def test_aggregate():
    data = {"scores": [10, 20, 30, 40, 50]}
    assert evaluate("scores.aggregate($total + $this, 0)", data) == 150


def test_date_literal():
    assert evaluate("@2024-01-15") == "2024-01-15"


def test_time_literal():
    assert evaluate("@T14:30:00") == "14:30:00"


def test_quantity_literal():
    assert evaluate("10 'mg'") == {"value": 10.0, "unit": "mg"}


def test_pydantic_model():
    from pydantic import BaseModel

    class Patient(BaseModel):
        resourceType: str = "Patient"
        id: str
        active: bool

    patient = Patient(id="pat-42", active=True)
    assert evaluate("active", patient.model_dump()) is True


def test_is_integer():
    assert evaluate("(1).is(Integer)") is True


def test_is_decimal():
    assert evaluate("(1.5).is(Decimal)") is True


def test_to_string():
    assert evaluate("42.toString()") == "42"


def test_to_integer():
    assert evaluate("'123'.toInteger()") == 123
