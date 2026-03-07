import fhirpathrs


def test_none_passthrough():
    assert fhirpathrs.evaluate("val", {"val": None}) is None


def test_bool_true():
    assert fhirpathrs.evaluate("val", {"val": True}) is True


def test_bool_false():
    assert fhirpathrs.evaluate("val", {"val": False}) is False


def test_int_preserved():
    result = fhirpathrs.evaluate("val", {"val": 42})
    assert result == 42
    assert isinstance(result, int)


def test_float_preserved():
    result = fhirpathrs.evaluate("val", {"val": 3.14})
    assert result == 3.14
    assert isinstance(result, float)


def test_string_preserved():
    result = fhirpathrs.evaluate("val", {"val": "hello"})
    assert result == "hello"
    assert isinstance(result, str)


def test_list_preserved():
    result = fhirpathrs.evaluate("val", {"val": [1, 2, 3]})
    assert result == [1, 2, 3]


def test_dict_preserved():
    result = fhirpathrs.evaluate("val", {"val": {"a": 1, "b": "two"}})
    assert result == {"a": 1, "b": "two"}


def test_nested_collections():
    data = {"matrix": [[1, 2], [3, 4]]}
    result = fhirpathrs.evaluate("matrix", data)
    assert result == [[1, 2], [3, 4]]


def test_integer_arithmetic_returns_int():
    result = fhirpathrs.evaluate("2 + 3")
    assert result == 5
    assert isinstance(result, int)


def test_decimal_arithmetic_returns_float():
    result = fhirpathrs.evaluate("1.5 + 2.5")
    assert result == 4.0
    # 4.0 has fract() == 0, so it comes back as int
    assert isinstance(result, int)


def test_date_as_string():
    result = fhirpathrs.evaluate("@2024-01-15")
    assert result == "2024-01-15"
    assert isinstance(result, str)


def test_datetime_as_string():
    result = fhirpathrs.evaluate("@2024-01-15T10:30:00Z")
    assert isinstance(result, str)
    assert "2024-01-15" in result
    assert "10:30:00" in result


def test_time_as_string():
    result = fhirpathrs.evaluate("@T14:30:00")
    assert isinstance(result, str)
    assert "14:30:00" in result


def test_quantity_as_dict():
    result = fhirpathrs.evaluate("10 'mg'")
    assert isinstance(result, dict)
    assert result["value"] == 10.0
    assert result["unit"] == "mg"
