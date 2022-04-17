import pytest
import rastro
from rastro import AS2RAD

def test_from_c04_file(iau2000_c04_14_filepath):
    eop = rastro.EarthOrientationData.from_c04_file(iau2000_c04_14_filepath, "Zero", True)

    assert eop.eop_type == "C04"
    assert eop.len() == 21877
    assert eop.mjd_min == 37665
    assert eop.mjd_max == 59541
    assert eop.extrapolate == "Zero"
    assert eop.interpolate is True


def test_from_default_c04():
    eop = rastro.EarthOrientationData.from_default_c04("Zero", True)

    assert eop.eop_type == "C04"
    assert eop.len() == 21877
    assert eop.mjd_min == 37665
    assert eop.mjd_max == 59541
    assert eop.extrapolate == "Zero"
    assert eop.interpolate is True


def test_from_standard_file_bulletin_a(iau2000_finals_ab_filepath):
    eop = rastro.EarthOrientationData.from_standard_file(iau2000_finals_ab_filepath, "Hold", True,
                                                         "StandardBulletinA")

    assert eop.len() == 18261
    assert eop.mjd_min == 41684
    assert eop.mjd_max == 59944
    assert eop.eop_type == "StandardBulletinA"
    assert eop.extrapolate == "Hold"
    assert eop.interpolate is True
    assert eop.mjd_last_lod == 59570
    assert eop.mjd_last_dxdy == 59648


def test_from_default_standard_bulletin_a():
    eop = rastro.EarthOrientationData.from_default_standard("Hold", True, "StandardBulletinA")

    assert eop.len() != 0
    assert eop.mjd_min == 41684
    assert eop.mjd_max >= 59944
    assert eop.eop_type == "StandardBulletinA"
    assert eop.extrapolate == "Hold"
    assert eop.interpolate is True
    assert eop.mjd_last_lod >= 59570
    assert eop.mjd_last_dxdy >= 59648


def test_from_standard_file_bulletin_b(iau2000_finals_ab_filepath):
    eop = rastro.EarthOrientationData.from_standard_file(iau2000_finals_ab_filepath, "Error", True,
                                                         "StandardBulletinB")

    assert eop.len() == 17836
    assert eop.mjd_min == 41684
    assert eop.mjd_max == 59519
    assert eop.eop_type == "StandardBulletinB"
    assert eop.extrapolate == "Error"
    assert eop.interpolate is True
    assert eop.mjd_last_lod == 0
    assert eop.mjd_last_dxdy == 59519


def test_from_default_standard_bulletin_b():
    eop = rastro.EarthOrientationData.from_default_standard("Error", True, "StandardBulletinB")

    assert eop.len() != 0
    assert eop.mjd_min == 41684
    assert eop.mjd_max >= 59519
    assert eop.eop_type == "StandardBulletinB"
    assert eop.extrapolate == "Error"
    assert eop.interpolate is True
    assert eop.mjd_last_lod == 0
    assert eop.mjd_last_dxdy >= 59519

def test_get_ut1_utc(iau2000_finals_ab_filepath):
    eop = rastro.EarthOrientationData.from_standard_file(
            iau2000_finals_ab_filepath, "Hold", True, "StandardBulletinA"
    )

    # Test getting exact point in table
    assert eop.get_ut1_utc(59569.0) == -0.1079838

    # Test interpolating within table
    assert eop.get_ut1_utc(59569.5) == (-0.1079838 + -0.1075832)/2.0

    # Test extrapolation hold
    assert eop.get_ut1_utc(59950.0) == -0.0278563

    # Test extrapolation zero
    eop = rastro.EarthOrientationData.from_standard_file(
        iau2000_finals_ab_filepath, "Zero", True, "StandardBulletinA"
    )
    assert eop.get_ut1_utc(59950.0) == 0.0


def test_get_pm_xy(iau2000_finals_ab_filepath):
    eop = rastro.EarthOrientationData.from_standard_file(
        iau2000_finals_ab_filepath, "Hold", True, "StandardBulletinA"
    )

    # Test getting exact point in table
    pm_x, pm_y = eop.get_pm(59569.0)
    assert pm_x == 0.075367*AS2RAD
    assert pm_y == 0.263430*AS2RAD

    # Test interpolating within table
    pm_x, pm_y = eop.get_pm(59569.5)
    assert pm_x == (0.075367*AS2RAD + 0.073151*AS2RAD)/2.0
    assert pm_y == (0.263430*AS2RAD + 0.264294*AS2RAD)/2.0

    # Test extrapolation hold
    pm_x, pm_y = eop.get_pm(59950.0)
    assert pm_x == 0.096178*AS2RAD
    assert pm_y == 0.252770*AS2RAD

    # Test extrapolation zero
    eop = rastro.EarthOrientationData.from_standard_file(
        iau2000_finals_ab_filepath, "Zero", True, "StandardBulletinA"
    )
    pm_x, pm_y = eop.get_pm(59950.0)
    assert pm_x == 0.0
    assert pm_y == 0.0


def test_get_dxdy(iau2000_finals_ab_filepath):
    eop = rastro.EarthOrientationData.from_standard_file(
        iau2000_finals_ab_filepath, "Hold", True, "StandardBulletinA"
    )

    # Test getting exact point in table
    dX, dY = eop.get_dxdy(59569.0)
    assert dX == 0.088*AS2RAD
    assert dY == 0.057*AS2RAD

    # Test interpolating within table
    dX, dY = eop.get_dxdy(59569.5)
    assert dX == (0.088*AS2RAD + 0.086*AS2RAD)/2.0
    assert dY == (0.057*AS2RAD + 0.058*AS2RAD)/2.0

    # Test extrapolation hold
    dX, dY = eop.get_dxdy(59950.0)
    assert dX == 0.283*AS2RAD
    assert dY == 0.104*AS2RAD

    # Test extrapolation zero
    eop = rastro.EarthOrientationData.from_standard_file(
        iau2000_finals_ab_filepath, "Zero", True, "StandardBulletinA"
    )
    dX, dY = eop.get_dxdy(59950.0)
    assert dX == 0.0
    assert dY == 0.0


def test_get_lod(iau2000_finals_ab_filepath):
    eop = rastro.EarthOrientationData.from_standard_file(
        iau2000_finals_ab_filepath, "Hold", True, "StandardBulletinA"
    )

    # Test getting exact point in table
    assert eop.get_lod(59569.0) == -0.4288

    # Test interpolating within table
    assert eop.get_lod(59569.5) == (-0.4288 + -0.3405)/2.0

    # Test extrapolation hold
    assert eop.get_lod(59950.0) == -0.3405

    # Test extrapolation zero
    eop = rastro.EarthOrientationData.from_standard_file(
        iau2000_finals_ab_filepath, "Zero", True, "StandardBulletinA"
    )
    assert eop.get_lod(59950.0) == 0.0


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
