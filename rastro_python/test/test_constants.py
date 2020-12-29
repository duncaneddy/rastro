import pytest
import rastro
import math

def test_deg2rad():
    assert rastro.DEG2RAD == math.pi/180.0

def test_DEG2RAD():
    assert rastro.DEG2RAD == math.pi/180.0

def test_RAD2DEG():
    assert rastro.RAD2DEG == 180.0/math.pi

def test_AS2RAD():
    assert rastro.AS2RAD == rastro.DEG2RAD / 3600.0

def test_RAD2AS():
    assert rastro.RAD2AS == rastro.RAD2DEG * 3600.0

def test_MJD_ZERO():
    assert rastro.MJD_ZERO == 2400000.5

def test_MJD2000():
    assert rastro.MJD2000 == 51544.5

def test_GPS_TAI():
    assert rastro.GPS_TAI == -19.0

def test_TAI_GPS ():
    assert rastro.TAI_GPS  == -rastro.GPS_TAI

def test_TT_TAI():
    assert rastro.TT_TAI == 32.184

def test_TAI_TT():
    assert rastro.TAI_TT == -rastro.TT_TAI

def test_GPS_TT():
    assert rastro.GPS_TT == rastro.GPS_TAI + rastro.TAI_TT

def test_TT_GPS():
    assert rastro.TT_GPS == -rastro.GPS_TT

def test_GPS_ZERO():
    assert rastro.GPS_ZERO == 44244.0

def test_C_LIGHT():
    assert rastro.C_LIGHT == 299792458.0

def test_AU():
    assert rastro.AU == 1.49597870700e11

def test_R_EARTH():
    assert rastro.R_EARTH == 6.378136300e6

def test_WGS84_A():
    assert rastro.WGS84_A == 6378137.0

def test_WGS84_F():
    assert rastro.WGS84_F == 1.0/298.257223563

def test_GM_EARTH():
    assert rastro.GM_EARTH == 3.986004415e14

def test_ECC_EARTH():
    assert rastro.ECC_EARTH == 8.1819190842622e-2

def test_J2_EARTH():
    assert rastro.J2_EARTH == 0.0010826358191967

def test_OMEGA_EARTH():
    assert rastro.OMEGA_EARTH == 7.292115146706979e-5

def test_GM_SUN():
    assert rastro.GM_SUN == 132712440041.939400*1e9

def test_R_SUN():
    assert rastro.R_SUN == 6.957*1e8

def test_P_SUN():
    assert rastro.P_SUN == 4.560E-6

def test_R_MOON():
    assert rastro.R_MOON == 1738*1e3

def test_GM_MOON():
    assert rastro.GM_MOON == 4902.800066*1e9

def test_GM_MERCURY():
    assert rastro.GM_MERCURY == 22031.780000*1e9

def test_GM_VENUS():
    assert rastro.GM_VENUS == 324858.592000*1e9

def test_GM_MARS():
    assert rastro.GM_MARS == 42828.37521*1e9

def test_GM_JUPITER():
    assert rastro.GM_JUPITER == 126712764.8*1e9

def test_GM_SATURN():
    assert rastro.GM_SATURN == 37940585.2*1e9

def test_GM_URANUS():
    assert rastro.GM_URANUS == 5794548.6*1e9

def test_GM_NEPTUNE():
    assert rastro.GM_NEPTUNE == 6836527.100580*1e9

def test_GM_PLUTO():
    assert rastro.GM_PLUTO == 977.000000*1e9
