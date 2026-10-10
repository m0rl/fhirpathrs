import pytest
from fhirpathrs import evaluate


def make_flat_list(n):
    return [i for i in range(n)]


def make_object_list(n):
    return {
        "items": [
            {"name": f"item_{i}", "value": i, "active": i % 2 == 0}
            for i in range(n)
        ]
    }


def make_nested_object(depth):
    obj = {"value": 42}
    for i in range(depth):
        obj = {f"level{depth - i}": obj}
    return obj


def make_deeply_nested_tree(depth):
    node = {"value": depth}
    for i in range(depth - 1, -1, -1):
        node = {"value": i, "item": node}
    return node


def make_fhir_bundle(n_entries):
    return {
        "resourceType": "Bundle",
        "entry": [
            {
                "resource": {
                    "resourceType": "Observation" if i % 3 == 0 else "Patient",
                    "status": "final",
                    "id": f"r{i}",
                    "value": i * 10,
                }
            }
            for i in range(n_entries)
        ],
    }


def make_observation_bundle(n_entries):
    narrative = "Heart rate recorded during routine vital signs check. " * 20
    return {
        "resourceType": "Bundle",
        "type": "searchset",
        "entry": [
            {
                "fullUrl": f"urn:uuid:obs-{i}",
                "resource": {
                    "resourceType": "Observation",
                    "id": f"obs-{i}",
                    "meta": {
                        "versionId": "1",
                        "lastUpdated": "2026-01-01T00:00:00Z",
                        "profile": [
                            "http://hl7.org/fhir/StructureDefinition/vitalsigns"
                        ],
                    },
                    "text": {
                        "status": "generated",
                        "div": f'<div xmlns="http://www.w3.org/1999/xhtml">{narrative}</div>',
                    },
                    "status": "amended" if i % 4 == 0 else "final",
                    "category": [
                        {
                            "coding": [
                                {
                                    "system": "http://terminology.hl7.org/CodeSystem/observation-category",
                                    "code": "vital-signs",
                                    "display": "Vital Signs",
                                }
                            ]
                        }
                    ],
                    "code": {
                        "coding": [
                            {
                                "system": "http://loinc.org",
                                "code": "8867-4",
                                "display": "Heart rate",
                            }
                        ],
                        "text": "Heart rate",
                    },
                    "subject": {
                        "reference": f"Patient/p{i % 50}",
                        "display": f"Patient {i % 50}",
                    },
                    "effectiveDateTime": "2026-01-01T10:00:00Z",
                    "valueQuantity": {
                        "value": 60 + i % 40,
                        "unit": "beats/minute",
                        "system": "http://unitsofmeasure.org",
                        "code": "/min",
                    },
                },
                "search": {"mode": "match"},
            }
            for i in range(n_entries)
        ],
    }


PATIENT = {
    "resourceType": "Patient",
    "id": "example",
    "active": True,
    "name": [
        {"family": "Smith", "given": ["John", "Jacob"]},
        {"family": "Smith-Jones", "given": ["Jane"]},
    ],
    "address": [
        {"city": "Springfield", "state": "IL", "postalCode": "62704"}
    ],
}

MEDIUM_DATA = {
    "name": "test",
    "a": {"b": {"c": {"d": {"e": 99}}}},
    "items": [
        {"name": f"item_{i}", "value": i, "active": i % 3 == 0}
        for i in range(50)
    ],
}


class TestDataConversion:
    def test_scalar(self, benchmark):
        benchmark(evaluate, "$this", 42)

    def test_small_object(self, benchmark):
        data = {"a": 1, "b": "hello", "c": True, "d": 3.14, "e": None}
        benchmark(evaluate, "a", data)

    def test_nested_object(self, benchmark):
        data = make_nested_object(3)
        benchmark(evaluate, "level1.level2.level3.value", data)

    def test_large_flat_list(self, benchmark):
        data = make_flat_list(1000)
        benchmark(evaluate, "count()", data)

    def test_large_object_list(self, benchmark):
        data = make_object_list(100)
        benchmark(evaluate, "items.count()", data)


class TestExpressionComplexity:
    def test_field_access(self, benchmark):
        benchmark(evaluate, "name", MEDIUM_DATA)

    def test_deep_path(self, benchmark):
        benchmark(evaluate, "a.b.c.d.e", MEDIUM_DATA)

    def test_filter(self, benchmark):
        benchmark(evaluate, "items.where(value > 25)", MEDIUM_DATA)

    def test_arithmetic(self, benchmark):
        benchmark(evaluate, "1 + 2 * 3 - 4")

    def test_string_length(self, benchmark):
        benchmark(evaluate, "name.length()", MEDIUM_DATA)

    def test_chained_ops(self, benchmark):
        benchmark(evaluate, "items.where(active = true).count()", MEDIUM_DATA)

    def test_aggregate(self, benchmark):
        data = {"items": list(range(50))}
        benchmark(evaluate, "items.aggregate($total + $this, 0)", data)


COLLECTION_SIZES = [10, 100, 1000]


class TestScaling:
    @pytest.mark.parametrize("n", COLLECTION_SIZES)
    def test_where_filter(self, benchmark, n):
        data = make_object_list(n)
        benchmark(evaluate, "items.where(value > 50)", data)

    @pytest.mark.parametrize("n", COLLECTION_SIZES)
    def test_count(self, benchmark, n):
        data = make_object_list(n)
        benchmark(evaluate, "items.count()", data)

    @pytest.mark.parametrize("n", COLLECTION_SIZES)
    def test_select(self, benchmark, n):
        data = make_object_list(n)
        benchmark(evaluate, "items.select(name)", data)

    @pytest.mark.parametrize("n", COLLECTION_SIZES)
    def test_repeat(self, benchmark, n):
        data = make_object_list(n)
        benchmark(evaluate, "items.repeat(value)", data)

    @pytest.mark.parametrize("n", COLLECTION_SIZES)
    def test_repeat_all(self, benchmark, n):
        data = make_object_list(n)
        benchmark(evaluate, "items.repeatAll(value)", data)


class TestDeepNesting:
    def test_where_chain_100(self, benchmark):
        expr = "true" + ".where(true)" * 100
        benchmark(evaluate, expr, True)

    def test_iif_nested_100(self, benchmark):
        expr = "42"
        for _ in range(100):
            expr = f"iif(true, {expr}, 0)"
        benchmark(evaluate, expr, None)

    def test_repeat_deep_tree(self, benchmark):
        tree = make_deeply_nested_tree(50)
        benchmark(evaluate, "repeat(item).count()", tree)

    def test_repeat_all_deep_tree(self, benchmark):
        tree = make_deeply_nested_tree(50)
        benchmark(evaluate, "repeatAll(item).count()", tree)


class TestConstants:
    def test_no_constants(self, benchmark):
        benchmark(evaluate, "items.where(value > 50).count()", MEDIUM_DATA)

    def test_one_constant(self, benchmark):
        constants = {"threshold": 50}
        benchmark(
            evaluate,
            "items.where(value > %threshold).count()",
            MEDIUM_DATA,
            constants,
        )

    def test_ten_constants(self, benchmark):
        constants = {f"var{i}": i * 10 for i in range(10)}
        benchmark(
            evaluate,
            "items.where(value > %var5).count()",
            MEDIUM_DATA,
            constants,
        )


class TestFHIRWorkloads:
    def test_patient_field(self, benchmark):
        benchmark(evaluate, "name.family", PATIENT)

    def test_patient_nested(self, benchmark):
        benchmark(evaluate, "name.where(family = 'Smith').given", PATIENT)

    def test_patient_exists(self, benchmark):
        benchmark(evaluate, "name.exists() and active", PATIENT)

    def test_bundle_filter_count(self, benchmark):
        bundle = make_fhir_bundle(100)
        benchmark(
            evaluate,
            "entry.resource.where(resourceType = 'Observation').value.count()",
            bundle,
        )

    def test_bundle_filter_select(self, benchmark):
        bundle = make_fhir_bundle(100)
        benchmark(
            evaluate,
            "entry.resource.where(resourceType = 'Observation').select(value)",
            bundle,
        )

    def test_bundle_filter_project_distinct(self, benchmark):
        bundle = make_fhir_bundle(100)
        benchmark(
            evaluate,
            "entry.resource.where(resourceType = 'Observation').select(value).distinct()",
            bundle,
        )


OBSERVATION_BUNDLE = make_observation_bundle(1000)


class TestObservationBundle:
    def test_status_filter_ids(self, benchmark):
        benchmark(
            evaluate,
            "entry.resource.where(status = 'final').id.count()",
            OBSERVATION_BUNDLE,
        )

    def test_value_threshold_subjects(self, benchmark):
        benchmark(
            evaluate,
            "entry.resource.where(valueQuantity.value > 90).subject.reference",
            OBSERVATION_BUNDLE,
        )

    def test_return_resources(self, benchmark):
        benchmark(evaluate, "entry.resource", OBSERVATION_BUNDLE)

    def test_first_resource(self, benchmark):
        benchmark(evaluate, "entry.first().resource", OBSERVATION_BUNDLE)

    def test_descendants_count(self, benchmark):
        benchmark(evaluate, "descendants().count()", OBSERVATION_BUNDLE)
