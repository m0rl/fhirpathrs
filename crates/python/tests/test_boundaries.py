import fhirpathrs


def test_low_boundary_decimal_default():
    result = fhirpathrs.evaluate("1.587.lowBoundary()")
    assert result == 1.5865


def test_high_boundary_decimal_default():
    result = fhirpathrs.evaluate("1.587.highBoundary()")
    assert result == 1.5875


def test_low_boundary_with_precision():
    result = fhirpathrs.evaluate("1.587.lowBoundary(2)")
    assert result == 1.58


def test_high_boundary_with_precision():
    result = fhirpathrs.evaluate("1.587.highBoundary(2)")
    assert result == 1.59


def test_low_boundary_negative():
    result = fhirpathrs.evaluate("(-1.587).lowBoundary()")
    assert result == -1.5875


def test_high_boundary_negative():
    result = fhirpathrs.evaluate("(-1.587).highBoundary()")
    assert result == -1.5865


def test_low_boundary_integer():
    result = fhirpathrs.evaluate("1.lowBoundary()")
    assert result == 0.5


def test_high_boundary_integer():
    result = fhirpathrs.evaluate("1.highBoundary()")
    assert result == 1.5


def test_boundary_precision_out_of_range_returns_empty():
    result = fhirpathrs.evaluate("1.587.lowBoundary(30)")
    assert result == []


def test_low_boundary_quantity():
    result = fhirpathrs.evaluate("1.587 'cm'.lowBoundary()")
    assert isinstance(result, dict)
    assert result["value"] == 1.5865
    assert result["unit"] == "cm"


def test_high_boundary_quantity():
    result = fhirpathrs.evaluate("1.587 'cm'.highBoundary()")
    assert isinstance(result, dict)
    assert result["value"] == 1.5875
    assert result["unit"] == "cm"


def test_low_boundary_date():
    result = fhirpathrs.evaluate("@2014-01-25.lowBoundary()")
    assert result == "2014-01-25T00:00:00.000"


def test_high_boundary_date():
    result = fhirpathrs.evaluate("@2014-01-25.highBoundary()")
    assert result == "2014-01-25T23:59:59.999"


def test_low_boundary_year():
    result = fhirpathrs.evaluate("@2014.lowBoundary()")
    assert result == "2014-01-01T00:00:00.000"


def test_high_boundary_year():
    result = fhirpathrs.evaluate("@2014.highBoundary()")
    assert result == "2014-12-31T23:59:59.999"
