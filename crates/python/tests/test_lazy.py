import pytest
import fhirpathrs


def test_returned_dict_is_the_input_object():
    address = {"city": "Springfield", "line": ["1 Main St"], "period": {"start": "2020"}}
    assert fhirpathrs.evaluate("address", {"address": address}) is address


def test_filtered_resources_are_the_input_objects():
    bundle = {
        "entry": [
            {"resource": {"resourceType": "Patient", "id": "a"}},
            {"resource": {"resourceType": "Observation", "id": "b", "valueQuantity": {"value": 0.123456789123}}},
        ]
    }
    result = fhirpathrs.evaluate("entry.resource.where(resourceType = 'Observation')", bundle)
    assert len(result) == 1
    assert result[0] is bundle["entry"][1]["resource"]


def test_unsupported_value_in_untouched_subtree_is_not_converted():
    data = {"id": "1", "meta": {"tag": object()}}
    assert fhirpathrs.evaluate("id", data) == "1"


def test_unsupported_value_in_touched_field_raises_type_error():
    data = {"id": "1", "meta": {"tag": object()}}
    with pytest.raises(TypeError):
        fhirpathrs.evaluate("meta.tag", data)


def test_out_of_range_int_in_touched_field_raises_value_error():
    with pytest.raises(ValueError):
        fhirpathrs.evaluate("a.b", {"a": {"b": 10**31}})


def test_non_string_key_in_loaded_object_raises_type_error():
    with pytest.raises(TypeError):
        fhirpathrs.evaluate("a.children()", {"a": {1: "x"}})


def test_data_error_takes_precedence_over_evaluation_error():
    data = {"meta": {"tag": object()}, "items": [1, 2]}
    with pytest.raises(TypeError):
        fhirpathrs.evaluate("(meta.tag | items).single()", data)


def test_dict_constant_fields_are_read_lazily():
    constants = {"config": {"threshold": 2, "unused": object()}}
    data = {"items": [1, 2, 3]}
    assert fhirpathrs.evaluate("items.where($this > %config.threshold)", data, constants) == [3]


def test_deeply_nested_input_is_traversed_without_recursion():
    depth = 10_000
    node = {"value": depth}
    for i in range(depth - 1, -1, -1):
        node = {"value": i, "item": node}
    assert fhirpathrs.evaluate("repeatAll(item).count()", node) == depth


def test_equal_input_objects_are_deduplicated():
    data = {"a": {"code": "x", "system": "s"}, "b": {"system": "s", "code": "x"}}
    assert fhirpathrs.evaluate("(a | b).count()", data) == 1
