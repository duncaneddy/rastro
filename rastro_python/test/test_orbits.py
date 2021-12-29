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
    vp = rastro.periapsis_velocity(rastro.R_MOON + 500e3, 0.001, rastro.GM_MOON)
    assert vp == pytest.approx(1481.5842246768275, abs=1e-12)

def test_apogee_velocity():
    vp = rastro.apogee_velocity(rastro.R_EARTH + 500e3, 0.001)
    assert vp == pytest.approx(7604.999751676446, abs=1e-12)

def test_apogee_velocity_general():
    vp = rastro.apoapsis_velocity(rastro.R_MOON + 500e3, 0.001, rastro.GM_MOON)
    assert vp == pytest.approx(1478.624016435715, abs=1e-12)

def test_sun_synchronous_inclination():
    vp = rastro.sun_synchronous_inclination(rastro.R_EARTH + 500e3, 0.001, True)
    assert vp == pytest.approx(97.40172901366881, abs=1e-12)

def test_anomaly_eccentric_to_mean():
    # 0 
    M = rastro.anomaly_eccentric_to_mean(0.0, 0.0, as_degrees=False)
    assert M == 0

    M = rastro.anomaly_eccentric_to_mean(0.0, 0.0, as_degrees=True)
    assert M == 0

    # 180
    M = rastro.anomaly_eccentric_to_mean(math.pi/2, 0.1, as_degrees=False)
    assert M == pytest.approx(1.4707963267948965, abs=1e-12)

    M = rastro.anomaly_eccentric_to_mean(90.0, 0.1, as_degrees=True)
    assert M == pytest.approx(84.27042204869177, abs=1e-3)

    # 180
    M = rastro.anomaly_eccentric_to_mean(math.pi, 0.0, as_degrees=False)
    assert M == pytest.approx(math.pi, abs=1e-12)

    M = rastro.anomaly_eccentric_to_mean(180.0, 0.0, as_degrees=True)
    assert M == 180.0

def test_anomaly_mean_to_eccentric():
    # 0 
    E = rastro.anomaly_mean_to_eccentric(0.0, 0.0, as_degrees=False)
    assert E == 0

    E = rastro.anomaly_mean_to_eccentric(0.0, 0.0, as_degrees=True)
    assert E == 0

    # 180
    E = rastro.anomaly_mean_to_eccentric(1.4707963267948965, 0.1, as_degrees=False)
    assert E == pytest.approx(math.pi/2, abs=1e-12)

    E = rastro.anomaly_mean_to_eccentric(84.27042204869177, 0.1, as_degrees=True)
    assert E == pytest.approx(90.0, abs=1e-12)

    # 180
    E = rastro.anomaly_mean_to_eccentric(math.pi, 0.0, as_degrees=False)
    assert E == pytest.approx(math.pi, abs=1e-12)

    E = rastro.anomaly_mean_to_eccentric(180.0, 0.0, as_degrees=True)
    assert E == 180.0

    # Large Eccentricities
    E = rastro.anomaly_mean_to_eccentric(180.0, 0.9, as_degrees=True)
    assert E == 180.0

def test_anomaly_true_to_eccentric():
    # 0 degrees
    anm_ecc = rastro.anomaly_true_to_eccentric(0.0, 0.0, False)
    assert anm_ecc == 0.0

    anm_ecc = rastro.anomaly_true_to_eccentric(0.0, 0.0, True)
    assert anm_ecc == 0.0

    # 180 degrees
    anm_ecc = rastro.anomaly_true_to_eccentric(math.pi, 0.0, False)
    assert anm_ecc == math.pi

    anm_ecc = rastro.anomaly_true_to_eccentric(180.0, 0.0, True)
    assert anm_ecc == 180.0

    # 90 degrees
    anm_ecc = rastro.anomaly_true_to_eccentric(math.pi / 2.0, 0.0, False)
    assert anm_ecc == pytest.approx(math.pi/2.0, abs=1e-12);

    anm_ecc = rastro.anomaly_true_to_eccentric(90.0, 0.0, True)
    assert anm_ecc == pytest.approx(90.0, abs=1e-12);

    anm_ecc = rastro.anomaly_true_to_eccentric(math.pi / 2.0, 0.1, False)
    assert anm_ecc == pytest.approx(1.4706289056333368, abs=1e-12)

    anm_ecc = rastro.anomaly_true_to_eccentric(90.0, 0.1, True)
    assert anm_ecc == pytest.approx(84.26082952273322, abs=1e-12)

def test_anomaly_eccentric_to_true():
    # 0 degrees
    anm_true = rastro.anomaly_eccentric_to_true(0.0, 0.0, False)
    assert anm_true == 0.0

    anm_true = rastro.anomaly_eccentric_to_true(0.0, 0.0, True)
    assert anm_true == 0.0

    # 180 degrees
    anm_true = rastro.anomaly_eccentric_to_true(math.pi, 0.0, False)
    assert anm_true == math.pi

    anm_true = rastro.anomaly_eccentric_to_true(180.0, 0.0, True)
    assert anm_true == 180.0

    # 90 degrees
    anm_true = rastro.anomaly_eccentric_to_true(math.pi/2.0, 0.0, False)
    assert anm_true == pytest.approx(math.pi/2.0, abs=1e-12);

    anm_true = rastro.anomaly_eccentric_to_true(90.0, 0.0, True)
    assert anm_true == pytest.approx(90.0, abs=1e-12);

    anm_true = rastro.anomaly_eccentric_to_true(math.pi/2.0, 0.1, False)
    assert anm_true == pytest.approx(1.6709637479564563, abs=1e-12)

    anm_true = rastro.anomaly_eccentric_to_true(90.0, 0.1, True)
    assert anm_true == pytest.approx(95.73917047726677, abs=1e-12)

def test_anomaly_true_to_mean():
    # 0 degrees
    m = rastro.anomaly_true_to_mean(0.0, 0.0, False)
    assert m == 0.0

    m = rastro.anomaly_true_to_mean(0.0, 0.0, True)
    assert m == 0.0

    # 180 degrees
    m = rastro.anomaly_true_to_mean(math.pi, 0.0, False)
    assert m == math.pi

    m = rastro.anomaly_true_to_mean(180.0, 0.0, True)
    assert m == 180.0

    # 90 degrees
    m = rastro.anomaly_true_to_mean(math.pi / 2.0, 0.1, False)
    assert m == pytest.approx(1.3711301619226748, abs=1e-12)

    m = rastro.anomaly_true_to_mean(90.0, 0.1, True)
    assert m == pytest.approx(78.55997144125844, abs=1e-12)

def test_anomaly_mean_to_true():
    # 0 degrees
    nu = rastro.anomaly_mean_to_true(0.0, 0.0, False)
    assert nu == 0.0

    nu = rastro.anomaly_mean_to_true(0.0, 0.0, True)
    assert nu == 0.0

    # 180 degrees
    nu = rastro.anomaly_mean_to_true(math.pi, 0.0, False)
    assert nu == math.pi

    nu = rastro.anomaly_mean_to_true(180.0, 0.0, True)
    assert nu == 180.0

    # 90 degrees
    nu = rastro.anomaly_mean_to_true(math.pi/2.0, 0.1, False)
    assert nu == pytest.approx(1.7694813731148669, abs=1e-12)

    nu = rastro.anomaly_mean_to_true(90.0, 0.1, True)
    assert nu == pytest.approx(101.38381460649556, abs=1e-12)