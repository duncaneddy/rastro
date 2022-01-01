import pytest
import rastro

def test_from_c04_file():
    pass

def test_from_default_c04():
    eop = rastro.EarthOrientationData.from_default_c04("Zero", True)

    print(eop)

def test_parse_standard_eop_line_bulletin_a():
    pass

def test_parse_standard_eop_line_bulletin_b():
    pass

def test_from_standard_file_bulletin_a():
    pass

def test_from_default_standard_bulletin_a():
    pass

def test_from_standard_file_bulletin_b():
    pass

def test_from_default_standard_bulletin_b():
    pass

def test_get_ut1_utc():
    pass

def test_get_pm_xy():
    pass

def test_get_dxdy():
    pass

def test_get_lod():
    pass

def test_eop_extrapolation_error():
    pass
