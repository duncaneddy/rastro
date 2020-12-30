import pytest
import math
import rastro

def test_orbital_period():
    T = rastro.orbital_period(rastro.R_EARTH + 500e3)
    assert T == pytest.approx(5676.977164028288, abs=1e-12)

def test_orbital_period_general():
    a  = rastro.R_EARTH + 500e3
    gm = rastro.GM_EARTH
    T  = rastro.orbital_period_general(a, gm)

    assert T == pytest.approx(5676.977164028288, abs=1e-12)

def test_orbital_period_general_moon():
    a  = rastro.R_MOON + 500e3
    gm = rastro.GM_MOON
    T  = rastro.orbital_period_general(a, gm)
    
    assert T == pytest.approx(9500.531451174307, abs=1e-12)

def test_mean_motion():
    n = rastro.mean_motion(rastro.R_EARTH + 500e3, as_degrees=False)
    assert n == pytest.approx(0.0011067836148773837, abs=1e-12)

    n = rastro.mean_motion(rastro.R_EARTH + 500e3, as_degrees=True)
    assert n == pytest.approx(0.0634140299667068, abs=1e-12)

    n = rastro.mean_motion(rastro.R_EARTH + 500e3)
    assert n == pytest.approx(0.0634140299667068, abs=1e-12)

def test_mean_motion_general():
    n = rastro.mean_motion_general(rastro.R_EARTH + 500e3, rastro.GM_MOON, as_degrees=False)
    assert n != pytest.approx(0.0011067836148773837, abs=1e-12)

    n = rastro.mean_motion_general(rastro.R_EARTH + 500e3, rastro.GM_MOON, as_degrees=True)
    assert n != pytest.approx(0.0634140299667068, abs=1e-12)

def test_semimajor_axis():
    a = rastro.semimajor_axis(0.0011067836148773837, as_degrees=False)
    assert a == pytest.approx(rastro.R_EARTH + 500e3, abs=1e-6)

    a = rastro.semimajor_axis(0.0634140299667068, as_degrees=True)
    assert a == pytest.approx(rastro.R_EARTH + 500e3, abs=1e-6)

    a = rastro.semimajor_axis(0.0634140299667068)
    assert a == pytest.approx(rastro.R_EARTH + 500e3, abs=1e-6)

def test_semimajor_axis():
    a = rastro.semimajor_axis_general(0.0011067836148773837, rastro.GM_MOON, as_degrees=False)
    assert a != pytest.approx(rastro.R_EARTH + 500e3, abs=1e-6)

    a = rastro.semimajor_axis_general(0.0634140299667068, rastro.GM_MOON, as_degrees=True)
    assert a != pytest.approx(rastro.R_EARTH + 500e3, abs=1e-6)

def test_perigee_velocity():
    vp = rastro.perigee_velocity(rastro.R_EARTH + 500e3, 0.001)
    assert vp == pytest.approx(7620.224976404526, abs=1e-12)

def test_perigee_velocity_general():
    vp = rastro.perigee_velocity_general(rastro.R_MOON + 500e3, 0.001, rastro.GM_MOON)
    assert vp == pytest.approx(1481.5842246768275, abs=1e-12)

def test_apogee_velocity():
    vp = rastro.apogee_velocity(rastro.R_EARTH + 500e3, 0.001)
    assert vp == pytest.approx(7604.999751676446, abs=1e-12)

def test_apogee_velocity_general():
    vp = rastro.apogee_velocity_general(rastro.R_MOON + 500e3, 0.001, rastro.GM_MOON)
    assert vp == pytest.approx(1478.624016435715, abs=1e-12)

def test_sunsync_inclination():
    vp = rastro.sunsync_inclination(rastro.R_EARTH + 500e3, 0.001, True)
    assert vp == pytest.approx(97.40172901366881, abs=1e-12)

def test_anmECCtoMEAN():
    # 0 
    M = rastro.anmECCtoMEAN(0.0, 0.0, as_degrees=False)
    assert M == 0

    M = rastro.anmECCtoMEAN(0.0, 0.0, as_degrees=True)
    assert M == 0

    # 180
    M = rastro.anmECCtoMEAN(math.pi/2, 0.1, as_degrees=False)
    assert M == pytest.approx(1.4707963267948965, abs=1e-12)

    M = rastro.anmECCtoMEAN(90.0, 0.1, as_degrees=True)
    assert M == pytest.approx(84.27042204869177, abs=1e-3)

    # 180
    M = rastro.anmECCtoMEAN(math.pi, 0.0, as_degrees=False)
    assert M == pytest.approx(math.pi, abs=1e-12)

    M = rastro.anmECCtoMEAN(180.0, 0.0, as_degrees=True)
    assert M == 180.0

def test_anmMEANtoECC():
    # 0 
    E = rastro.anmMEANtoECC(0.0, 0.0, as_degrees=False)
    assert E == 0

    E = rastro.anmMEANtoECC(0.0, 0.0, as_degrees=True)
    assert E == 0

    # 180
    E = rastro.anmMEANtoECC(1.4707963267948965, 0.1, as_degrees=False)
    assert E == pytest.approx(math.pi/2, abs=1e-12)

    E = rastro.anmMEANtoECC(84.27042204869177, 0.1, as_degrees=True)
    assert E == pytest.approx(90.0, abs=1e-12)

    # 180
    E = rastro.anmMEANtoECC(math.pi, 0.0, as_degrees=False)
    assert E == pytest.approx(math.pi, abs=1e-12)

    E = rastro.anmMEANtoECC(180.0, 0.0, as_degrees=True)
    assert E == 180.0

    # Large Eccentricities
    E = rastro.anmMEANtoECC(180.0, 0.9, as_degrees=True)
    assert E == 180.0