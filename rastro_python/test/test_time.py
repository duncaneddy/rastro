import pytest
import rastro

@pytest.fixture
def eop():
    return rastro.EarthOrientationData.from_default_c04("Hold", True)

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
    # assert rastro.time_system_offset(jd, 0.0, "GPS", "GPS", eop) == 0.0
    # assert rastro.time_system_offset(jd, 0.0, "GPS", "TT",  eop) == rastro.TT_GPS
    # assert rastro.time_system_offset(jd, 0.0, "GPS", "UTC", eop) == dutc + rastro.TAI_GPS
    # assert rastro.time_system_offset(jd, 0.0, "GPS", "UT1", eop) == pytest.approx(dutc + rastro.TAI_GPS + dut1, abs=1e-6)
    # assert rastro.time_system_offset(jd, 0.0, "GPS", "TAI", eop) == rastro.TAI_GPS
    #
    # // TT
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TT, TimeSystem::GPS, &eop), GPS_TT);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TT, TimeSystem::TT,  &eop), 0.0);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TT, TimeSystem::UTC, &eop), dutc + TAI_TT);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TT, TimeSystem::UT1, &eop), dutc + TAI_TT + dut1, epsilon=1e-6);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TT, TimeSystem::TAI, &eop), TAI_TT);
    #
    # // UTC
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UTC, TimeSystem::GPS, &eop), -dutc + GPS_TAI);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UTC, TimeSystem::TT,  &eop), -dutc + TT_TAI);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UTC, TimeSystem::UTC, &eop), 0.0);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UTC, TimeSystem::UT1, &eop), dut1, epsilon=1e-6);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UTC, TimeSystem::TAI, &eop), -dutc);
    #
    # // UT1
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UT1, TimeSystem::GPS, &eop), -dutc + GPS_TAI - dut1, epsilon=1e-6);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UT1, TimeSystem::TT,  &eop), -dutc + TT_TAI - dut1, epsilon=1e-6);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UT1, TimeSystem::UTC, &eop), -dut1, epsilon=1e-6);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UT1, TimeSystem::UT1, &eop), 0.0, epsilon=1e-6);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UT1, TimeSystem::TAI, &eop), -dutc - dut1, epsilon=1e-6);
    #
    # // TAI
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TAI, TimeSystem::GPS, &eop), GPS_TAI);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TAI, TimeSystem::TT,  &eop), TT_TAI);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TAI, TimeSystem::UTC, &eop), dutc);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TAI, TimeSystem::UT1, &eop), dutc + dut1, epsilon=1e-6);
    # assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TAI, TimeSystem::TAI, &eop), 0.0);

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


