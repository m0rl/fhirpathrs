"""Benchmarks for fhirpathpy (pure Python) — same test cases as bench_evaluate.py.

Run with:
    uv run pytest crates/python/tests/bench_fhirpathpy.py -v --benchmark-sort=mean

Compare against fhirpathrs:
    uv run pytest crates/python/tests/bench_evaluate.py crates/python/tests/bench_fhirpathpy.py -v --benchmark-sort=mean --benchmark-group-by=func
"""

import pytest
import fhirpathpy


def fp_evaluate(expression, data, constants=None):
    return fhirpathpy.evaluate(data, expression, constants or {})


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
    @pytest.mark.skip(reason="fhirpathpy does not support $this on scalars")
    def test_scalar(self, benchmark):
        benchmark(fp_evaluate, "$this", 42)

    def test_small_object(self, benchmark):
        data = {"a": 1, "b": "hello", "c": True, "d": 3.14, "e": None}
        benchmark(fp_evaluate, "a", data)

    def test_nested_object(self, benchmark):
        data = make_nested_object(3)
        benchmark(fp_evaluate, "level1.level2.level3.value", data)

    def test_large_flat_list(self, benchmark):
        data = make_flat_list(1000)
        benchmark(fp_evaluate, "count()", data)

    def test_large_object_list(self, benchmark):
        data = make_object_list(100)
        benchmark(fp_evaluate, "items.count()", data)


class TestExpressionComplexity:
    def test_field_access(self, benchmark):
        benchmark(fp_evaluate, "name", MEDIUM_DATA)

    def test_deep_path(self, benchmark):
        benchmark(fp_evaluate, "a.b.c.d.e", MEDIUM_DATA)

    def test_filter(self, benchmark):
        benchmark(fp_evaluate, "items.where(value > 25)", MEDIUM_DATA)

    def test_arithmetic(self, benchmark):
        benchmark(fp_evaluate, "1 + 2 * 3 - 4", {})

    def test_string_length(self, benchmark):
        benchmark(fp_evaluate, "name.length()", MEDIUM_DATA)

    def test_chained_ops(self, benchmark):
        benchmark(fp_evaluate, "items.where(active = true).count()", MEDIUM_DATA)

    def test_aggregate(self, benchmark):
        data = {"items": list(range(50))}
        benchmark(fp_evaluate, "items.aggregate($total + $this, 0)", data)


COLLECTION_SIZES = [10, 100, 1000]


class TestScaling:
    @pytest.mark.parametrize("n", COLLECTION_SIZES)
    def test_where_filter(self, benchmark, n):
        data = make_object_list(n)
        benchmark(fp_evaluate, "items.where(value > 50)", data)

    @pytest.mark.parametrize("n", COLLECTION_SIZES)
    def test_count(self, benchmark, n):
        data = make_object_list(n)
        benchmark(fp_evaluate, "items.count()", data)

    @pytest.mark.parametrize("n", COLLECTION_SIZES)
    def test_select(self, benchmark, n):
        data = make_object_list(n)
        benchmark(fp_evaluate, "items.select(name)", data)

    @pytest.mark.parametrize("n", COLLECTION_SIZES)
    def test_repeat(self, benchmark, n):
        data = make_object_list(n)
        benchmark(fp_evaluate, "items.repeat(value)", data)

    @pytest.mark.skip(reason="fhirpathpy does not implement repeatAll")
    @pytest.mark.parametrize("n", COLLECTION_SIZES)
    def test_repeat_all(self, benchmark, n):
        data = make_object_list(n)
        benchmark(fp_evaluate, "items.repeatAll(value)", data)


class TestDeepNesting:
    def test_where_chain_100(self, benchmark):
        expr = "true" + ".where(true)" * 100
        benchmark(fp_evaluate, expr, True)

    @pytest.mark.skip(reason="fhirpathpy hits recursion limit at 100-deep iif")
    def test_iif_nested_100(self, benchmark):
        expr = "42"
        for _ in range(100):
            expr = f"iif(true, {expr}, 0)"
        benchmark(fp_evaluate, expr, None)

    def test_repeat_deep_tree(self, benchmark):
        tree = make_deeply_nested_tree(50)
        benchmark(fp_evaluate, "repeat(item).count()", tree)

    @pytest.mark.skip(reason="fhirpathpy does not implement repeatAll")
    def test_repeat_all_deep_tree(self, benchmark):
        tree = make_deeply_nested_tree(50)
        benchmark(fp_evaluate, "repeatAll(item).count()", tree)


class TestConstants:
    def test_no_constants(self, benchmark):
        benchmark(fp_evaluate, "items.where(value > 50).count()", MEDIUM_DATA)

    def test_one_constant(self, benchmark):
        constants = {"threshold": 50}
        benchmark(
            fp_evaluate,
            "items.where(value > %threshold).count()",
            MEDIUM_DATA,
            constants,
        )

    def test_ten_constants(self, benchmark):
        constants = {f"var{i}": i * 10 for i in range(10)}
        benchmark(
            fp_evaluate,
            "items.where(value > %var5).count()",
            MEDIUM_DATA,
            constants,
        )


class TestFHIRWorkloads:
    def test_patient_field(self, benchmark):
        benchmark(fp_evaluate, "name.family", PATIENT)

    def test_patient_nested(self, benchmark):
        benchmark(fp_evaluate, "name.where(family = 'Smith').given", PATIENT)

    def test_patient_exists(self, benchmark):
        benchmark(fp_evaluate, "name.exists() and active", PATIENT)

    def test_bundle_filter_count(self, benchmark):
        bundle = make_fhir_bundle(100)
        benchmark(
            fp_evaluate,
            "entry.resource.where(resourceType = 'Observation').value.count()",
            bundle,
        )

    def test_bundle_filter_select(self, benchmark):
        bundle = make_fhir_bundle(100)
        benchmark(
            fp_evaluate,
            "entry.resource.where(resourceType = 'Observation').select(value)",
            bundle,
        )

    def test_bundle_filter_project_distinct(self, benchmark):
        bundle = make_fhir_bundle(100)
        benchmark(
            fp_evaluate,
            "entry.resource.where(resourceType = 'Observation').select(value).distinct()",
            bundle,
        )
