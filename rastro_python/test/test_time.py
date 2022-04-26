import pytest
import math
import rastro

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
    assert rastro.time_system_offset(jd, 0.0, "GPS", "GPS") == 0.0
    assert rastro.time_system_offset(jd, 0.0, "GPS", "TT")  == rastro.TT_GPS
    assert rastro.time_system_offset(jd, 0.0, "GPS", "UTC") == dutc + rastro.TAI_GPS
    assert rastro.time_system_offset(jd, 0.0, "GPS", "UT1") == pytest.approx(dutc + rastro.TAI_GPS + dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "GPS", "TAI") == rastro.TAI_GPS

    # TT
    assert rastro.time_system_offset(jd, 0.0, "TT", "GPS") == rastro.GPS_TT
    assert rastro.time_system_offset(jd, 0.0, "TT", "TT")  == 0.0
    assert rastro.time_system_offset(jd, 0.0, "TT", "UTC") == dutc + rastro.TAI_TT
    assert rastro.time_system_offset(jd, 0.0, "TT", "UT1") == pytest.approx(dutc + rastro.TAI_TT + dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "TT", "TAI") == rastro.TAI_TT

    # UTC
    assert rastro.time_system_offset(jd, 0.0, "UTC", "GPS") == -dutc + rastro.GPS_TAI
    assert rastro.time_system_offset(jd, 0.0, "UTC", "TT")  == -dutc + rastro.TT_TAI
    assert rastro.time_system_offset(jd, 0.0, "UTC", "UTC") == 0.0
    assert rastro.time_system_offset(jd, 0.0, "UTC", "UT1") == pytest.approx(dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "UTC", "TAI") == -dutc

    # UT1
    assert rastro.time_system_offset(jd, 0.0, "UT1", "GPS") == pytest.approx(-dutc + rastro.GPS_TAI - dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "UT1", "TT")  == pytest.approx(-dutc + rastro.TT_TAI - dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "UT1", "UTC") == pytest.approx(-dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "UT1", "UT1") == pytest.approx(0.0, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "UT1", "TAI") == pytest.approx(-dutc - dut1, abs=1e-6)

    # TAI
    assert rastro.time_system_offset(jd, 0.0, "TAI", "GPS") == rastro.GPS_TAI
    assert rastro.time_system_offset(jd, 0.0, "TAI", "TT")  == rastro.TT_TAI
    assert rastro.time_system_offset(jd, 0.0, "TAI", "UTC") == dutc
    assert rastro.time_system_offset(jd, 0.0, "TAI", "UT1") == pytest.approx(dutc + dut1, abs=1e-6)
    assert rastro.time_system_offset(jd, 0.0, "TAI", "TAI") == 0.0

def test_epoch_string(eop):
    epc = rastro.Epoch.from_date(2022, 1, 1, "GPS")

    assert str(epc) == "2022-01-01 00:00:00.000 GPS"

def test_epoch_repr(eop):
    epc = rastro.Epoch.from_date(2022, 1, 1, "GPS")

    assert epc.__repr__() == "Epoch<2459580, 43219, 0, 0, GPS>"

def test_epoch_time_system(eop):
    epc = rastro.Epoch.from_date(2022, 1, 1, "GPS")
    assert epc.time_system == "GPS"

    epc = rastro.Epoch.from_date(2022, 1, 1, "TAI")
    assert epc.time_system == "TAI"

    epc = rastro.Epoch.from_date(2022, 1, 1, "TT")
    assert epc.time_system == "TT"

    epc = rastro.Epoch.from_date(2022, 1, 1, "UTC")
    assert epc.time_system == "UTC"

    epc = rastro.Epoch.from_date(2022, 1, 1, "UT1")
    assert epc.time_system == "UT1"

def test_epoch_from_date(eop):
    epc = rastro.Epoch.from_date(2020, 1, 2, "GPS")

    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()

    assert year == 2020
    assert month == 1
    assert day == 2
    assert hour == 0
    assert minute == 0
    assert second == 0.0
    assert nanosecond == 0.0

def test_epoch_from_datetime(eop):
    epc = rastro.Epoch.from_datetime(2020, 1, 2, 3, 4, 5.0, 6.0, "GPS")

    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()

    assert year == 2020
    assert month == 1
    assert day == 2
    assert hour == 3
    assert minute == 4
    assert second == 5.0
    assert nanosecond == 6.0

def test_epoch_from_string(eop):
    epc = rastro.Epoch.from_string("2018-12-20")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2018
    assert month == 12
    assert day == 20
    assert hour == 0
    assert minute == 0
    assert second == 0.0
    assert nanosecond == 0.0
    assert epc.time_system == "UTC"

    epc = rastro.Epoch.from_string("2018-12-20T16:22:19.0Z")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2018
    assert month == 12
    assert day == 20
    assert hour == 16
    assert minute == 22
    assert second == 19.0
    assert nanosecond == 0.0
    assert epc.time_system == "UTC"

    epc = rastro.Epoch.from_string("2018-12-20T16:22:19.123Z")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2018
    assert month == 12
    assert day == 20
    assert hour == 16
    assert minute == 22
    assert second == 19.0
    assert nanosecond == 123000000.0
    assert epc.time_system == "UTC"

    epc = rastro.Epoch.from_string("2018-12-20T16:22:19.123456789Z")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2018
    assert month == 12
    assert day == 20
    assert hour == 16
    assert minute == 22
    assert second == 19.0
    assert nanosecond == 123456789.0
    assert epc.time_system == "UTC"

    epc = rastro.Epoch.from_string("2018-12-20T16:22:19Z")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2018
    assert month == 12
    assert day == 20
    assert hour == 16
    assert minute == 22
    assert second == 19.0
    assert nanosecond == 0.0
    assert epc.time_system == "UTC"

    epc = rastro.Epoch.from_string("20181220T162219Z")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2018
    assert month == 12
    assert day == 20
    assert hour == 16
    assert minute == 22
    assert second == 19.0
    assert nanosecond == 0.0
    assert epc.time_system == "UTC"

    epc = rastro.Epoch.from_string("2018-12-01 16:22:19 GPS")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2018
    assert month == 12
    assert day == 1
    assert hour == 16
    assert minute == 22
    assert second == 19.0
    assert nanosecond == 0.0
    assert epc.time_system == "GPS"

    epc = rastro.Epoch.from_string("2018-12-01 16:22:19.0 GPS")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2018
    assert month == 12
    assert day == 1
    assert hour == 16
    assert minute == 22
    assert second == 19.0
    assert nanosecond == 0.0
    assert epc.time_system == "GPS"

    epc = rastro.Epoch.from_string("2018-12-01 16:22:19.123 GPS")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2018
    assert month == 12
    assert day == 1
    assert hour == 16
    assert minute == 22
    assert second == 19.0
    assert nanosecond == 123000000.0
    assert epc.time_system == "GPS"

    epc = rastro.Epoch.from_string("2018-12-01 16:22:19.123456789 GPS")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2018
    assert month == 12
    assert day == 1
    assert hour == 16
    assert minute == 22
    assert second == 19.0
    assert nanosecond == 123456789.0
    assert epc.time_system == "GPS"

def test_epoch_from_jd(eop):
    epc = rastro.Epoch.from_jd(rastro.MJD_ZERO + rastro.MJD2000, "TAI")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2000
    assert month == 1
    assert day == 1
    assert hour == 12
    assert minute == 0
    assert second == 0.0
    assert nanosecond == 0.0
    assert epc.time_system == "TAI"

    epc = rastro.Epoch.from_jd(rastro.MJD_ZERO + rastro.MJD2000, "GPS")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime_as_tsys("TAI")
    assert year == 2000
    assert month == 1
    assert day == 1
    assert hour == 12
    assert minute == 0
    assert second == 19.0
    assert nanosecond == 17643.974853515625 # Rounding error from floating point conversion
    assert epc.time_system == "GPS"

def test_epoch_from_mjd(eop):
    epc = rastro.Epoch.from_mjd(rastro.MJD2000, "TAI")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2000
    assert month == 1
    assert day == 1
    assert hour == 12
    assert minute == 0
    assert second == 0.0
    assert nanosecond == 0.0
    assert epc.time_system == "TAI"

    epc = rastro.Epoch.from_mjd(rastro.MJD2000, "GPS")
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime_as_tsys("TAI")
    assert year == 2000
    assert month == 1
    assert day == 1
    assert hour == 12
    assert minute == 0
    assert second == 19.0
    assert nanosecond == 17643.974853515625 # Rounding error from floating point conversion
    assert epc.time_system == "GPS"

def test_epoch_from_gps_date():
    epc = rastro.Epoch.from_gps_date(0, 0.0)
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 1980
    assert month == 1
    assert day == 6
    assert hour == 0
    assert minute == 0
    assert second == 0.0
    assert nanosecond == 0.0
    assert epc.time_system == "GPS"

    epc = rastro.Epoch.from_gps_date(2194, 435781.5)
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 28
    assert hour == 1
    assert minute == 3
    assert second == 1.0
    assert nanosecond == 500000000.0
    assert epc.time_system == "GPS"

def test_epoch_from_gps_seconds():
    epc = rastro.Epoch.from_gps_seconds(0.0)
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 1980
    assert month == 1
    assert day == 6
    assert hour == 0
    assert minute == 0
    assert second == 0.0
    assert nanosecond == 0.0
    assert epc.time_system == "GPS"

    epc = rastro.Epoch.from_gps_seconds(2194.0 * 7.0 * 86400.0 + 3.0 * 3600.0 + 61.5)
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 23
    assert hour == 3
    assert minute == 1
    assert second == 1.0
    assert nanosecond == 500000000.0
    assert epc.time_system == "GPS"

def test_epoch_from_gps_nanoseconds():
    epc = rastro.Epoch.from_gps_nanoseconds(0)
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 1980
    assert month == 1
    assert day == 6
    assert hour == 0
    assert minute == 0
    assert second == 0.0
    assert nanosecond == 0.0
    assert epc.time_system == "GPS"

    gpsns = (2194 * 7 * 86400 + 3 * 3600 + 61) * 1000000000 + 1
    epc = rastro.Epoch.from_gps_nanoseconds(gpsns)
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 23
    assert hour == 3
    assert minute == 1
    assert second == 1.0
    assert nanosecond == 1.0
    assert epc.time_system == "GPS"

def test_epoch_to_jd(eop):
    epc = rastro.Epoch.from_datetime(2000, 1, 1, 12, 0, 0.0, 0.0, "TAI")

    assert epc.jd() == rastro.MJD_ZERO + rastro.MJD2000

    epc = rastro.Epoch.from_datetime(2000, 1, 1, 12, 0, 0.0, 0.0, "TAI")
    assert epc.jd_as_tsys("UTC") == rastro.MJD_ZERO + rastro.MJD2000 - 32.0 / 86400.0

def test_epoch_to_mjd(eop):
    epc = rastro.Epoch.from_datetime(2000, 1, 1, 12, 0, 0.0, 0.0, "TAI")

    assert epc.mjd() == rastro.MJD2000

    epc = rastro.Epoch.from_datetime(2000, 1, 1, 12, 0, 0.0, 0.0, "TAI")
    assert epc.mjd_as_tsys("UTC") == rastro.MJD2000 - 32.0 / 86400.0

def test_gps_date(eop):
    epc = rastro.Epoch.from_date(2018, 3, 1, "GPS")
    (gps_week, gps_seconds) = epc.gps_date()
    assert gps_week == 1990
    assert gps_seconds == 4.0 * 86400.0

    epc = rastro.Epoch.from_date(2018, 3, 8, "GPS")
    (gps_week, gps_seconds) = epc.gps_date()
    assert gps_week == 1991
    assert gps_seconds == 4.0 * 86400.0

    epc = rastro.Epoch.from_date(2018, 3, 11, "GPS")
    (gps_week, gps_seconds) = epc.gps_date()
    assert gps_week == 1992
    assert gps_seconds == 0.0 * 86400.0

    epc = rastro.Epoch.from_date(2018, 3, 24, "GPS")
    (gps_week, gps_seconds) = epc.gps_date()
    assert gps_week == 1993
    assert gps_seconds == 6.0 * 86400.0

def test_gps_seconds(eop):
    epc = rastro.Epoch.from_date(1980, 1, 6, "GPS")
    assert epc.gps_seconds() == 0.0

    epc = rastro.Epoch.from_datetime(1980, 1, 7, 0, 0, 1.0, 0.0, "GPS")
    assert epc.gps_seconds() == 86401.0

def test_gps_nanoseconds(eop):
    epc = rastro.Epoch.from_date(1980, 1, 6, "GPS")
    assert epc.gps_nanoseconds() == 0.0

    epc = rastro.Epoch.from_datetime(1980, 1, 7, 0, 0, 1.0, 0.0, "GPS")
    assert epc.gps_nanoseconds() == 86401.0 * 1.0e9

def test_isostring(eop):
    # Confirm Before the leap second
    epc = rastro.Epoch.from_datetime(2016, 12, 31, 23, 59, 59.0, 0.0, "UTC")
    assert epc.isostring() == "2016-12-31T23:59:59Z"

    # The leap second
    epc = rastro.Epoch.from_datetime(2016, 12, 31, 23, 59, 60.0, 0.0, "UTC")
    assert epc.isostring() == "2016-12-31T23:59:60Z"

    # After the leap second
    epc = rastro.Epoch.from_datetime(2017, 1, 1, 0, 0, 0.0, 0.0, "UTC")
    assert epc.isostring() == "2017-01-01T00:00:00Z"

def test_isostringd(eop):
    # Confirm Before the leap second
    epc = rastro.Epoch.from_datetime(2000, 1, 1, 12, 0, 1.23456, 0.0, "UTC")
    assert epc.isostringd(0) == "2000-01-01T12:00:01Z"
    assert epc.isostringd(1) == "2000-01-01T12:00:01.2Z"
    assert epc.isostringd(2) == "2000-01-01T12:00:01.23Z"
    assert epc.isostringd(3) == "2000-01-01T12:00:01.234Z"

def test_to_string_as_tsys(eop):
    # Confirm Before the leap second
    epc = rastro.Epoch.from_datetime(2020, 1, 1, 0, 0, 0.0, 0.0, "UTC")
    epc.to_string_as_tsys("UTC") == "2020-01-01 00:00:00.000 UTC"
    epc.to_string_as_tsys("GPS") == "2020-01-01 00:00:18.000 GPS"

def test_gmst(eop):
    epc = rastro.Epoch.from_date(2000, 1, 1, "UTC")
    assert epc.gmst(True) == pytest.approx(99.969, abs=1e-3)

    epc = rastro.Epoch.from_date(2000, 1, 1, "UTC")
    assert epc.gmst(False) == pytest.approx(99.969 * math.pi / 180.0, abs = 1.0e-3)

def test_gast(eop):
    epc = rastro.Epoch.from_date(2000, 1, 1, "UTC")
    assert epc.gast(True) == pytest.approx(99.965, abs = 1.0e-3)

    epc = rastro.Epoch.from_date(2000, 1, 1, "UTC")
    assert epc.gast(False) == pytest.approx(99.965 * math.pi / 180.0, abs = 1.0e-3)

def test_ops_add_assign():
    # Test Positive additions of different size
    epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")
    epc += 1.0
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 31
    assert hour == 0
    assert minute == 0
    assert second == 1.0
    assert nanosecond == 0.0
    assert epc.time_system == "TAI"

    epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")
    epc += 86400.5
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 2
    assert day == 1
    assert hour == 0
    assert minute == 0
    assert second == 0.0
    assert nanosecond == 500_000_000.0
    assert epc.time_system == "TAI"

    epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")
    epc += 1.23456789e-9
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 31
    assert hour == 0
    assert minute == 0
    assert second == 0.0
    assert nanosecond == 1.23456789
    assert epc.time_system == "TAI"

    # Test subtractions of different size
    epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")
    epc += -1.0
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 30
    assert hour == 23
    assert minute == 59
    assert second == 59.0
    assert nanosecond == 0.0
    assert epc.time_system == "TAI"

    epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")
    epc += -86400.5
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 29
    assert hour == 23
    assert minute == 59
    assert second == 59.0
    assert nanosecond == 500_000_000.0
    assert epc.time_system == "TAI"

    # Test types
    epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")
    epc += 1
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 31
    assert hour == 0
    assert minute == 0
    assert second == 1.0
    assert nanosecond == 0.0
    assert epc.time_system == "TAI"

    epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")
    epc += -1
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 30
    assert hour == 23
    assert minute == 59
    assert second == 59.0
    assert nanosecond == 0.0
    assert epc.time_system == "TAI"

def test_ops_sub_assign():
    epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")
    epc -= 1.23456789e-9
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 30
    assert hour == 23
    assert minute == 59
    assert second == 59.0
    assert nanosecond == 999_999_999.7654321
    assert epc.time_system == "TAI"

    # Test subtractions of different size
    epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")
    epc -= 1.0
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 30
    assert hour == 23
    assert minute == 59
    assert second == 59.0
    assert nanosecond == 0.0
    assert epc.time_system == "TAI"

    epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")
    epc -= 86400.5
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 29
    assert hour == 23
    assert minute == 59
    assert second == 59.0
    assert nanosecond == 500_000_000.0
    assert epc.time_system == "TAI"

    # Test types
    epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")
    epc -= 1
    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 30
    assert hour == 23
    assert minute == 59
    assert second == 59.0
    assert nanosecond == 0.0
    assert epc.time_system == "TAI"

def test_ops_add():
    # Base epoch
    epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")

    # Test Positive additions of different size
    epc_2 = epc + 1.0
    (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 31
    assert hour == 0
    assert minute == 0
    assert second == 1.0
    assert nanosecond == 0.0
    assert epc.time_system == "TAI"

    epc_2 = epc + 86400.5
    (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime()
    assert year == 2022
    assert month == 2
    assert day == 1
    assert hour == 0
    assert minute == 0
    assert second == 0.0
    assert nanosecond == 500_000_000.0
    assert epc.time_system == "TAI"

    epc_2 = epc + 1.23456789e-9
    (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 31
    assert hour == 0
    assert minute == 0
    assert second == 0.0
    assert nanosecond == 1.23456789
    assert epc.time_system == "TAI"

    # Test subtractions of different size
    epc_2 = epc + -1.0
    (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 30
    assert hour == 23
    assert minute == 59
    assert second == 59.0
    assert nanosecond == 0.0
    assert epc.time_system == "TAI"

    epc_2 = epc + -86400.5
    (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 29
    assert hour == 23
    assert minute == 59
    assert second == 59.0
    assert nanosecond == 500_000_000.0
    assert epc.time_system == "TAI"

    # Test types
    epc_2 = epc + 1
    (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 31
    assert hour == 0
    assert minute == 0
    assert second == 1.0
    assert nanosecond == 0.0
    assert epc.time_system == "TAI"

    epc_2 = epc + -1
    (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime()
    assert year == 2022
    assert month == 1
    assert day == 30
    assert hour == 23
    assert minute == 59
    assert second == 59.0
    assert nanosecond == 0.0
    assert epc.time_system == "TAI"

# def test_ops_sub(eop):
#     # Base epoch
#     epc = rastro.Epoch.from_date(2022, 1, 31, "TAI")
#
#     # Test subtractions of different size
#     epc_2 = epc - 1.0
#     (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime()
#     assert year == 2022
#     assert month == 1
#     assert day == 30
#     assert hour == 23
#     assert minute == 59
#     assert second == 59.0
#     assert nanosecond == 0.0
#     assert epc.time_system == "TAI"
#
#     epc_2 = epc - 86400.5
#     (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime()
#     assert year == 2022
#     assert month == 1
#     assert day == 29
#     assert hour == 23
#     assert minute == 59
#     assert second == 59.0
#     assert nanosecond == 500_000_000.0
#     assert epc.time_system == "TAI"
#
#     # Test types
#     epc_2 = epc - 1
#     (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime()
#     assert year == 2022
#     assert month == 1
#     assert day == 30
#     assert hour == 23
#     assert minute == 59
#     assert second == 59.0
#     assert nanosecond == 0.0
#     assert epc.time_system == "TAI"

def test_ops_sub_epoch():
    epc_1 = rastro.Epoch.from_date(2022, 1, 31, "TAI")
    epc_2 = rastro.Epoch.from_date(2022, 2, 1, "TAI")
    assert epc_2 - epc_1 == 86400.0

    epc_1 = rastro.Epoch.from_date(2021, 1, 1, "TAI")
    epc_2 = rastro.Epoch.from_date(2022, 1, 1, "TAI")
    assert epc_2 - epc_1 == 86400.0 * 365.0

    epc_1 = rastro.Epoch.from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, "TAI")
    epc_2 = rastro.Epoch.from_datetime(2022, 1, 1, 0, 0, 0.0, 1.0, "TAI")
    assert epc_2 - epc_1 == 1.0e-9

    epc_1 = rastro.Epoch.from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, "TAI")
    epc_2 = rastro.Epoch.from_datetime(2022, 1, 2, 1, 1, 1.0, 1.0, "TAI")
    assert epc_2 - epc_1 == 86400.0 + 3600.0 + 60.0 + 1.0 + 1.0e-9

    epc_1 = rastro.Epoch.from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, "TAI")
    epc_2 = rastro.Epoch.from_datetime(2022, 1, 1, 0, 0, 19.0, 0.0, "TAI")
    assert epc_2 - epc_1 == 19.0
    assert epc_1 - epc_2 == -19.0
    assert epc_1 - epc_1 == 0.0

def test_eq_epoch(eop):
    epc_1 = rastro.Epoch.from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23456789, "TAI")
    epc_2 = rastro.Epoch.from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23456789, "TAI")
    assert epc_1 == epc_2

    epc_1 = rastro.Epoch.from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23456, "TAI")
    epc_2 = rastro.Epoch.from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23455, "TAI")
    assert epc_1 != epc_2

    # Check instant comparison against time systems works
    epc_1 = rastro.Epoch.from_datetime(1980, 1, 6, 0, 0, 0.0, 0.0, "GPS")
    epc_2 = rastro.Epoch.from_datetime(1980, 1, 6, 0, 0, 19.0, 0.0, "TAI")
    assert epc_1 == epc_2

def test_cmp_epoch(eop):
    epc_1 = rastro.Epoch.from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23456, "TAI")
    epc_2 = rastro.Epoch.from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23455, "TAI")
    assert (epc_1 > epc_2) == True
    assert (epc_1 >= epc_2) == True
    assert (epc_1 < epc_2) == False
    assert (epc_1 <= epc_2) == False

    epc_1 = rastro.Epoch.from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23456, "TAI")
    epc_2 = rastro.Epoch.from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23456, "TAI")
    assert (epc_1 > epc_2) == False
    assert (epc_1 >= epc_2) == True
    assert (epc_1 < epc_2) == False
    assert (epc_1 <= epc_2) == True

# def test_nanosecond_addition_stability():
#     pass
#
def test_addition_stability():
    epc = rastro.Epoch.from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, "TAI")

    # Advance a year 1 second at a time
    for _ in range(0, 86400*365):
        epc += 1.0

    (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime()
    assert year == 2023
    assert month == 1
    assert day == 1
    assert hour == 0
    assert minute == 0
    assert second == 0.0
    assert nanosecond == 0.0

def test_epoch_range():
    epcs = rastro.Epoch.from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, "TAI")
    epcf = rastro.Epoch.from_datetime(2022, 1, 2, 0, 0, 0.0, 0.0, "TAI")
    epc  = rastro.Epoch.from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, "TAI")

    epcv = []
    for e in rastro.EpochRange(epcs, epcf, 1.0):
        assert epc == e
        epc += 1
        epcv.append(e)


    epcl = rastro.Epoch.from_datetime(2022, 1, 1, 23, 59, 59.0, 0.0, "TAI")
    assert len(epcv) == 86400
    assert (epcv[-1] != epcf)
    assert (epcv[-1] - epcl) <= 1.0e-9

