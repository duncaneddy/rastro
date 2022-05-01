import pytest
import rastro

def test_set_global_eop_from_zero():
    rastro.set_global_eop_from_zero()

    assert rastro.get_global_eop_initialization() == True
    assert rastro.get_global_eop_len() == 0
    assert rastro.get_global_eop_type() == "Static"
    assert rastro.get_global_eop_extrapolate() == "Zero"
    assert rastro.get_global_eop_interpolate() == False
    assert rastro.get_global_eop_mjd_min() == 0
    assert rastro.get_global_eop_mjd_max() == 0
    assert rastro.get_global_eop_mjd_last_lod() == 0
    assert rastro.get_global_eop_mjd_last_dxdy() == 0

def test_set_global_eop_from_static_values():
    rastro.set_global_eop_from_static_values(0.001, 0.002, 0.003, 0.004, 0.005, 0.006)

    assert rastro.get_global_eop_initialization() == True
    assert rastro.get_global_eop_len() == 1
    assert rastro.get_global_eop_type() == "Static"
    assert rastro.get_global_eop_extrapolate() == "Hold"
    assert rastro.get_global_eop_interpolate() == False
    assert rastro.get_global_eop_mjd_min() == 0
    assert rastro.get_global_eop_mjd_max() == 0
    assert rastro.get_global_eop_mjd_last_lod() == 0
    assert rastro.get_global_eop_mjd_last_dxdy() == 0

def test_from_c04_file(iau2000_c04_14_filepath):
    rastro.set_global_eop_from_c04_file(iau2000_c04_14_filepath, "Zero", True)

    assert rastro.get_global_eop_initialization() == True
    assert rastro.get_global_eop_len() == 21877
    assert rastro.get_global_eop_type() == "C04"
    assert rastro.get_global_eop_extrapolate() == "Zero"
    assert rastro.get_global_eop_interpolate() == True
    assert rastro.get_global_eop_mjd_min() == 37665
    assert rastro.get_global_eop_mjd_max() == 59541
    assert rastro.get_global_eop_mjd_last_lod() == 59541
    assert rastro.get_global_eop_mjd_last_dxdy() == 59541

def test_from_default_c04():
    rastro.set_global_eop_from_default_c04("Zero", True)

    assert rastro.get_global_eop_initialization() == True
    assert rastro.get_global_eop_len() == 21877
    assert rastro.get_global_eop_type() == "C04"
    assert rastro.get_global_eop_extrapolate() == "Zero"
    assert rastro.get_global_eop_interpolate() == True
    assert rastro.get_global_eop_mjd_min() == 37665
    assert rastro.get_global_eop_mjd_max() == 59541
    assert rastro.get_global_eop_mjd_last_lod() == 59541
    assert rastro.get_global_eop_mjd_last_dxdy() == 59541


def test_from_standard_file_bulletin_a(iau2000_finals_ab_filepath):
    rastro.set_global_eop_from_standard_file(iau2000_finals_ab_filepath, "Hold", True, "StandardBulletinA")

    assert rastro.get_global_eop_initialization() == True
    assert rastro.get_global_eop_len() == 18261
    assert rastro.get_global_eop_type() == "StandardBulletinA"
    assert rastro.get_global_eop_extrapolate() == "Hold"
    assert rastro.get_global_eop_interpolate() == True
    assert rastro.get_global_eop_mjd_min() == 41684
    assert rastro.get_global_eop_mjd_max() == 59944
    assert rastro.get_global_eop_mjd_last_lod() == 59570
    assert rastro.get_global_eop_mjd_last_dxdy() == 59648

def test_from_default_standard_bulletin_a():
    eop = rastro.set_global_eop_from_default_standard("Hold", True, "StandardBulletinA")

    assert rastro.get_global_eop_initialization() == True
    assert rastro.get_global_eop_len() != 0
    assert rastro.get_global_eop_type() == "StandardBulletinA"
    assert rastro.get_global_eop_extrapolate() == "Hold"
    assert rastro.get_global_eop_interpolate() == True
    assert rastro.get_global_eop_mjd_min() == 41684
    assert rastro.get_global_eop_mjd_max() >= 59944
    assert rastro.get_global_eop_mjd_last_lod() >= 59570
    assert rastro.get_global_eop_mjd_last_dxdy() >= 59648


def test_from_standard_file_bulletin_b(iau2000_finals_ab_filepath):
    rastro.set_global_eop_from_standard_file(iau2000_finals_ab_filepath, "Error", True, "StandardBulletinB")

    assert rastro.get_global_eop_initialization() == True
    assert rastro.get_global_eop_len() == 17836
    assert rastro.get_global_eop_type() == "StandardBulletinB"
    assert rastro.get_global_eop_extrapolate() == "Error"
    assert rastro.get_global_eop_interpolate() == True
    assert rastro.get_global_eop_mjd_min() == 41684
    assert rastro.get_global_eop_mjd_max() == 59519
    assert rastro.get_global_eop_mjd_last_lod() == 0
    assert rastro.get_global_eop_mjd_last_dxdy() == 59519

def test_from_default_standard_bulletin_b():
    rastro.set_global_eop_from_default_standard("Error", True, "StandardBulletinB")

    assert rastro.get_global_eop_initialization() == True
    assert rastro.get_global_eop_len() != 0
    assert rastro.get_global_eop_type() == "StandardBulletinB"
    assert rastro.get_global_eop_extrapolate() == "Error"
    assert rastro.get_global_eop_interpolate() == True
    assert rastro.get_global_eop_mjd_min() == 41684
    assert rastro.get_global_eop_mjd_max() >= 59519
    assert rastro.get_global_eop_mjd_last_lod() == 0
    assert rastro.get_global_eop_mjd_last_dxdy() >= 59519

def test_get_ut1_utc(iau2000_finals_ab_filepath):
    rastro.set_global_eop_from_standard_file(iau2000_finals_ab_filepath, "Hold", True, "StandardBulletinA")

    # Test getting exact point in table
    assert rastro.get_global_ut1_utc(59569.0) == -0.1079838

    # Test interpolating within table
    assert rastro.get_global_ut1_utc(59569.5) == (-0.1079838 + -0.1075832)/2.0

    # Test extrapolation hold
    assert rastro.get_global_ut1_utc(59950.0) == -0.0278563

    # Test extrapolation zero
    rastro.set_global_eop_from_standard_file(iau2000_finals_ab_filepath, "Zero", True, "StandardBulletinA")
    assert rastro.get_global_ut1_utc(59950.0) == 0.0


def test_get_pm_xy(iau2000_finals_ab_filepath):
    rastro.set_global_eop_from_standard_file(iau2000_finals_ab_filepath, "Hold", True, "StandardBulletinA")

    # Test getting exact point in table
    pm_x, pm_y = rastro.get_global_pm(59569.0)
    assert pm_x == 0.075367*rastro.AS2RAD
    assert pm_y == 0.263430*rastro.AS2RAD

    # Test interpolating within table
    pm_x, pm_y = rastro.get_global_pm(59569.5)
    assert pm_x == (0.075367*rastro.AS2RAD + 0.073151*rastro.AS2RAD)/2.0
    assert pm_y == (0.263430*rastro.AS2RAD + 0.264294*rastro.AS2RAD)/2.0

    # Test extrapolation hold
    pm_x, pm_y = rastro.get_global_pm(59950.0)
    assert pm_x == 0.096178*rastro.AS2RAD
    assert pm_y == 0.252770*rastro.AS2RAD

    # Test extrapolation zero
    rastro.set_global_eop_from_standard_file(iau2000_finals_ab_filepath, "Zero", True, "StandardBulletinA")
    pm_x, pm_y = rastro.get_global_pm(59950.0)
    assert pm_x == 0.0
    assert pm_y == 0.0


def test_get_dxdy(iau2000_finals_ab_filepath):
    rastro.set_global_eop_from_standard_file(iau2000_finals_ab_filepath, "Hold", True, "StandardBulletinA")

    # Test getting exact point in table
    dX, dY = rastro.get_global_dxdy(59569.0)
    assert dX == pytest.approx(0.088*rastro.AS2RAD * 1.0e-3, abs=1e-12)
    assert dY == pytest.approx(0.057*rastro.AS2RAD * 1.0e-3, abs=1e-12)

    # Test interpolating within table
    dX, dY = rastro.get_global_dxdy(59569.5)
    assert dX == pytest.approx((0.088*rastro.AS2RAD + 0.086*rastro.AS2RAD)/2.0 * 1.0e-3, abs=1e-12)
    assert dY == pytest.approx((0.057*rastro.AS2RAD + 0.058*rastro.AS2RAD)/2.0 * 1.0e-3, abs=1e-12)

    # Test extrapolation hold
    dX, dY = rastro.get_global_dxdy(59950.0)
    assert dX == pytest.approx(0.283*rastro.AS2RAD * 1.0e-3, abs=1e-12)
    assert dY == pytest.approx(0.104*rastro.AS2RAD * 1.0e-3, abs=1e-12)

    # Test extrapolation zero
    rastro.set_global_eop_from_standard_file(iau2000_finals_ab_filepath, "Zero", True, "StandardBulletinA")
    dX, dY = rastro.get_global_dxdy(59950.0)
    assert dX == 0.0
    assert dY == 0.0


def test_get_lod(iau2000_finals_ab_filepath):
    rastro.set_global_eop_from_standard_file(iau2000_finals_ab_filepath, "Hold", True, "StandardBulletinA")

    # Test getting exact point in table
    assert rastro.get_global_lod(59569.0) == -0.4288 * 1.0e-3

    # Test interpolating within table
    assert rastro.get_global_lod(59569.5) == (-0.4288 + -0.3405)/2.0 * 1.0e-3

    # Test extrapolation hold
    assert rastro.get_global_lod(59950.0) == -0.3405 * 1.0e-3

    # Test extrapolation zero
    rastro.set_global_eop_from_standard_file(iau2000_finals_ab_filepath, "Zero", True, "StandardBulletinA")
    assert rastro.get_global_lod(59950.0) == 0.0


# TODO: Fix being able to run this text. It runs and properly raises a pyo3_runtiem.PanicException
#   which is uncatchable
# def test_eop_extrapolation_error(iau2000_finals_ab_filepath):
#     eop = rastro.EarthOrientationData.from_standard_file(
#         iau2000_finals_ab_filepath, "Error", True, "StandardBulletinA"
#     )
#
#     # This will raise an un-catchable panic exception
#     with pytest.raises(Exception):
#         eop.get_ut1_utc(59950.0)
