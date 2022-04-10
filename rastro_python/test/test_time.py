import pytest
import rastro

@pytest.fixture
def eop():
    return rastro.EarthOrientationData.from_default_c04("Hold", True)
#
def test_datetime_to_jd():
    assert rastro.datetime_to_jd(2000, 1, 1, 12, 0, 0.0, 0.0) == 2451545.0

def test_datetime_to_mjd():
    assert rastro.datetime_to_mjd(2000, 1, 1, 12, 0, 0.0, 0.0) == 51544.5

def test_jd_to_datetime():
    assert rastro.jd_to_datetime(2451545.0) == (2000, 1, 1, 12, 0, 0.0, 0.0)

def test_mjd_to_datetime():
    assert rastro.mjd_to_datetime(51544.5) == (2000, 1, 1, 12, 0, 0.0, 0.0)

def test_time_system_offset(eop):

    # Test date
    jd = rastro.datetime_to_jd(2018, 6, 1, 0, 0, 0.0, 0.0)

    # UTC - TAI offset
    dutc = -37.0
    dut1 = 0.0769859

    # GPS
    assert rastro.time_system_offset(jd, 0.0, "GPS", "GPS", eop) == 0.0
    assert rastro.time_system_offset(jd, 0.0, "GPS", "TT",  eop) == rastro.TT_GPS
    assert rastro.time_system_offset(jd, 0.0, "GPS", "UTC", eop) == dutc + rastro.TAI_GPS
    assert rastro.time_system_offset(jd, 0.0, "GPS", "UT1", eop) == pytest.approx(dutc + rastro.TAI_GPS + dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "GPS", "TAI", eop) == rastro.TAI_GPS

    # TT
    assert rastro.time_system_offset(jd, 0.0, "TT", "GPS", eop) == rastro.GPS_TT
    assert rastro.time_system_offset(jd, 0.0, "TT", "TT",  eop) == 0.0
    assert rastro.time_system_offset(jd, 0.0, "TT", "UTC", eop) == dutc + rastro.TAI_TT
    assert rastro.time_system_offset(jd, 0.0, "TT", "UT1", eop) == pytest.approx(dutc + rastro.TAI_TT + dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "TT", "TAI", eop) == rastro.TAI_TT

    # UTC
    assert rastro.time_system_offset(jd, 0.0, "UTC", "GPS", eop) == -dutc + rastro.GPS_TAI
    assert rastro.time_system_offset(jd, 0.0, "UTC", "TT",  eop) == -dutc + rastro.TT_TAI
    assert rastro.time_system_offset(jd, 0.0, "UTC", "UTC", eop) == 0.0
    assert rastro.time_system_offset(jd, 0.0, "UTC", "UT1", eop) == pytest.approx(dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "UTC", "TAI", eop) == -dutc

    # UT1
    assert rastro.time_system_offset(jd, 0.0, "UT1", "GPS", eop) == pytest.approx(-dutc + rastro.GPS_TAI - dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "UT1", "TT",  eop) == pytest.approx(-dutc + rastro.TT_TAI - dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "UT1", "UTC", eop) == pytest.approx(-dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "UT1", "UT1", eop) == pytest.approx(0.0, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "UT1", "TAI", eop) == pytest.approx(-dutc - dut1, abs=1e-6)

    # TAI
    assert rastro.time_system_offset(jd, 0.0, "TAI", "GPS", eop) == rastro.GPS_TAI
    assert rastro.time_system_offset(jd, 0.0, "TAI", "TT",  eop) == rastro.TT_TAI
    assert rastro.time_system_offset(jd, 0.0, "TAI", "UTC", eop) == dutc
    assert rastro.time_system_offset(jd, 0.0, "TAI", "UT1", eop) == pytest.approx(dutc + dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "TAI", "TAI", eop) == 0.0

# def test_epoch_string():
#     pass
#
# def test_epoch_repr():
#     pass
#
# def test_epoch_from_date():
#     pass
#
# def test_epoch_from_datetime():
#     pass
#
# def test_epoch_from_string():
#     pass
#
# def test_epoch_from_jd():
#     pass
#
# def test_epoch_from_mjd():
#     pass
#
# def test_epoch_from_gps_date():
#     pass
#
# def test_epoch_from_gps_seconds():
#     pass
#
# def test_epoch_from_gps_nanoseconds():
#     pass
#
# def test_epoch_to_jd():
#     pass
#
# def test_epoch_to_mjd():
#     pass
#
# def test_gps_date():
#     pass
#
# def test_gps_seconds():
#     pass
#
# def test_gps_nanoseconds():
#     pass
#
# def test_isostring():
#     pass
#
# def test_isostringd():
#     pass
#
# def test_to_string_as_tsys():
#     pass
#
# def test_gmst():
#     pass
#
# def test_gast():
#     pass
#
# def test_ops_add_assign():
#     pass
#
# def test_ops_sub_assign():
#     pass
#
# def test_ops_add():
#     pass
#
# def test_ops_sub():
#     pass
#
# def test_ops_sub_epoch():
#     pass
#
# def test_eq_epoch():
#     pass
#
# def test_cmp_epoch():
#     pass
#
# def test_nanosecond_addition_stability():
#     pass
#
# def test_addition_stability():
#     pass
#
# def test_epoch_range():
#     pass


