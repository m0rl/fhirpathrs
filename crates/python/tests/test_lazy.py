import multiprocessing
import sys

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


def test_member_access_converts_only_the_visited_path():
    data = {"item": {"item": {"value": 8, "unused": object()}, "unused": object()}, "unused": object()}
    assert fhirpathrs.evaluate("item.item.value", data) == 8


def test_type_check_converts_only_resource_type():
    data = {"item": {"resourceType": "Node", "unused": object()}}
    assert fhirpathrs.evaluate("item.is(Node)", data) is True


def test_missing_field_converts_one_level_only():
    data = {"a": {"x": object()}}
    assert fhirpathrs.evaluate("absent", data) == []


def test_comparison_converts_only_top_level_of_deep_input():
    node = {"value": 0, "unused": object()}
    for i in range(1, 100_000):
        node = {"value": i, "item": node}
    assert fhirpathrs.evaluate("$this = 5", node) is False


def test_deeply_nested_input_is_traversed_without_recursion():
    depth = 100_000
    node = {"value": depth}
    for i in range(depth - 1, -1, -1):
        node = {"value": i, "item": node}
    assert fhirpathrs.evaluate("repeatAll(item).count()", node) == depth


def test_evaluation_releases_input_dicts():
    leaf = {"value": 0}
    root = leaf
    for i in range(1, 5_000):
        root = {"value": i, "item": root}
    before = (sys.getrefcount(root), sys.getrefcount(leaf))
    assert fhirpathrs.evaluate("repeatAll(item).count()", root) == 4_999
    assert fhirpathrs.evaluate("descendants().count()", root) == 9_999
    assert (sys.getrefcount(root), sys.getrefcount(leaf)) == before


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


def repeat_ignores_unread_self_referencing_list():
    unused = []
    unused.append(unused)
    data = {"item": {"id": "one", "item": None, "unused": unused}}
    assert fhirpathrs.evaluate("repeat(item).count()", data) == 1


def repeat_ignores_unread_mutually_referencing_lists():
    unused = []
    other = [unused]
    unused.append(other)
    data = {"item": {"id": "one", "item": None, "unused": unused}}
    assert fhirpathrs.evaluate("repeat(item).count()", data) == 1


def reading_self_referencing_list_raises_value_error():
    unused = []
    unused.append(unused)
    with pytest.raises(ValueError, match="Circular reference detected"):
        fhirpathrs.evaluate("unused", {"unused": unused})


def reading_mutually_referencing_lists_raises_value_error():
    unused = []
    other = [unused]
    unused.append(other)
    with pytest.raises(ValueError, match="Circular reference detected"):
        fhirpathrs.evaluate("unused", {"unused": unused})


def repeat_skips_unread_shared_list_graph_of_single_result():
    unused = [1]
    for _ in range(59):
        unused = [unused, unused]
    data = {"item": {"id": "one", "item": None, "unused": unused}}
    assert fhirpathrs.evaluate("repeat(item).count()", data) == 1


def repeat_skips_unread_shared_list_graph_below_mismatched_shapes():
    unused = [1]
    for _ in range(59):
        unused = [unused, unused]
    data = {
        "item": [
            {"id": "a", "item": None, "meta": {"unused": unused}},
            {"id": "b", "item": None},
        ]
    }
    assert fhirpathrs.evaluate("repeat(item).count()", data) == 2


@pytest.mark.parametrize(
    "check",
    [
        repeat_ignores_unread_self_referencing_list,
        repeat_ignores_unread_mutually_referencing_lists,
        reading_self_referencing_list_raises_value_error,
        reading_mutually_referencing_lists_raises_value_error,
        repeat_skips_unread_shared_list_graph_of_single_result,
        repeat_skips_unread_shared_list_graph_below_mismatched_shapes,
    ],
    ids=lambda check: check.__name__,
)
def test_conversion_terminates(check):
    process = multiprocessing.get_context("spawn").Process(target=check)
    process.start()
    process.join(timeout=5)
    finished = not process.is_alive()
    process.kill()
    process.join()
    assert finished, "did not finish within 5 s"
    assert process.exitcode == 0, "failed in the child process, see captured stderr"


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
