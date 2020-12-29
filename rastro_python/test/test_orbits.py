import pytest
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