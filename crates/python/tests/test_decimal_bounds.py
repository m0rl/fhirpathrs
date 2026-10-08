import math

import pytest

import fhirpathrs


def test_int_beyond_i128_raises_value_error():
    with pytest.raises(ValueError):
        fhirpathrs.evaluate("$this", 10**100)


def test_int_beyond_decimal_range_raises_value_error():
    with pytest.raises(ValueError):
        fhirpathrs.evaluate("$this", 10**31)


def test_non_finite_float_raises_value_error():
    with pytest.raises(ValueError):
        fhirpathrs.evaluate("$this", math.inf)
    with pytest.raises(ValueError):
        fhirpathrs.evaluate("$this", math.nan)


def test_float_beyond_decimal_range_raises_value_error():
    with pytest.raises(ValueError):
        fhirpathrs.evaluate("$this", 1e301)


def test_in_range_values_round_trip():
    assert fhirpathrs.evaluate("$this", 10**20) == 10**20
    assert fhirpathrs.evaluate("$this + 0.1", 0.2) == 0.3
    assert fhirpathrs.evaluate("$this * 1000", 10**20) == 10**23
