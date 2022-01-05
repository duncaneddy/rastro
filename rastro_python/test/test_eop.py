import pytest
import rastro

def test_from_c04_file(iau2000_c04_14_filepath):
    eop = rastro.EarthOrientationData.from_c04_file("Zero", True)

    assert eop.data.len() == 21877
    assert eop.mjd_min, 37665
    assert eop.mjd_max, 59541
    assert eop.eop_type, EOPType::C04
    assert eop.extrapolate, EOPExtrapolation::Hold
    assert eop.interpolate, true

def test_from_default_c04():
    eop = rastro.EarthOrientationData.from_default_c04("Zero", True)

def test_from_standard_file_bulletin_a():
    pass

def test_from_default_standard_bulletin_a():
    eop = rastro.EarthOrientationData.from_default_standard("Zero", True, "A")

def test_from_standard_file_bulletin_b():
    pass

def test_from_default_standard_bulletin_b():
    eop = rastro.EarthOrientationData.from_default_standard("Zero", True, "B")

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
