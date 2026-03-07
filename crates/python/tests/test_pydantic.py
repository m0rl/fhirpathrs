import fhirpathrs
from pydantic import BaseModel


class HumanName(BaseModel):
    given: list[str]
    family: str


class Patient(BaseModel):
    resourceType: str = "Patient"
    id: str
    active: bool
    name: list[HumanName]


class Observation(BaseModel):
    resourceType: str = "Observation"
    status: str
    value: float


def test_simple_pydantic():
    patient = Patient(
        id="123",
        active=True,
        name=[HumanName(given=["Jane"], family="Doe")],
    )
    assert fhirpathrs.evaluate("active", patient.model_dump()) is True


def test_pydantic_nested():
    patient = Patient(
        id="456",
        active=False,
        name=[
            HumanName(given=["John", "James"], family="Smith"),
            HumanName(given=["Johnny"], family="Smith"),
        ],
    )
    data = patient.model_dump()
    assert fhirpathrs.evaluate("name.family", data) == ["Smith", "Smith"]
    assert fhirpathrs.evaluate("name.given", data) == ["John", "James", "Johnny"]


def test_pydantic_field_access():
    obs = Observation(status="final", value=98.6)
    data = obs.model_dump()
    assert fhirpathrs.evaluate("status", data) == "final"
    assert fhirpathrs.evaluate("value", data) == 98.6


def test_pydantic_with_constants():
    patient = Patient(
        id="789",
        active=True,
        name=[HumanName(given=["Alice"], family="Wong")],
    )
    result = fhirpathrs.evaluate(
        "name.where(family = %target).given",
        patient.model_dump(),
        constants={"target": "Wong"},
    )
    assert result == ["Alice"]
