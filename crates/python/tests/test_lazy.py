import multiprocessing

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


@pytest.mark.parametrize("unread", [{"unused": object()}, {"unused": float("nan")}, {7: "x"}])
def test_repeat_does_not_convert_unread_fields_of_its_results(unread):
    data = {"item": {"id": "one", "item": None, **unread}}
    assert fhirpathrs.evaluate("repeat(item).count()", data) == 1
    assert fhirpathrs.evaluate("repeat(item).id", data) == fhirpathrs.evaluate("repeatAll(item).id", data)


@pytest.mark.parametrize(("unused", "error"), [(object(), TypeError), (float("nan"), ValueError)])
def test_repeat_result_field_access_still_raises_conversion_error(unused, error):
    data = {"item": {"id": "one", "item": None, "unused": unused}}
    with pytest.raises(error):
        fhirpathrs.evaluate("repeat(item).unused", data)


def self_referencing_list():
    unused = []
    unused.append(unused)
    return unused


def mutually_referencing_lists():
    unused = []
    unused.append([unused])
    return unused


def repeat_ignores_unread_cyclic_list(make_cycle):
    data = {"item": {"id": "one", "item": None, "unused": make_cycle()}}
    assert fhirpathrs.evaluate("repeat(item).count()", data) == 1


def reading_cyclic_list_raises_value_error(make_cycle):
    with pytest.raises(ValueError, match="Circular reference detected"):
        fhirpathrs.evaluate("unused", {"unused": make_cycle()})


@pytest.mark.parametrize("make_cycle", [self_referencing_list, mutually_referencing_lists])
@pytest.mark.parametrize("check", [repeat_ignores_unread_cyclic_list, reading_cyclic_list_raises_value_error])
def test_cyclic_list_conversion_terminates(check, make_cycle):
    process = multiprocessing.get_context("spawn").Process(target=check, args=(make_cycle,))
    process.start()
    process.join(timeout=5)
    finished = not process.is_alive()
    process.kill()
    process.join()
    assert finished, f"{check.__name__} did not finish within 5 s"
    assert process.exitcode == 0, f"{check.__name__} failed in the child process, see captured stderr"


def test_shared_acyclic_list_converts_like_separate_copies():
    shared = [1, 2]
    assert fhirpathrs.evaluate("unused", {"unused": [shared, shared]}) == fhirpathrs.evaluate(
        "unused", {"unused": [[1, 2], [1, 2]]}
    )
    data = {"item": {"id": "one", "item": None, "unused": [shared, shared]}}
    assert fhirpathrs.evaluate("repeat(item).count()", data) == 1


def test_repeat_deduplicates_equal_input_objects():
    tree = {"item": [{"id": "a", "item": None}, {"id": "b", "item": {"id": "a", "item": None}}]}
    assert fhirpathrs.evaluate("repeat(item).id", tree) == ["a", "b"]
