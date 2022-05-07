import pytest
import numpy as np
import rastro
from pytest import approx

@pytest.fixture()
def static_eop():
    pm_x = 0.0349282 * rastro.AS2RAD
    pm_y = 0.4833163 * rastro.AS2RAD
    ut1_utc = -0.072073685
    dX = 0.0001750 * rastro.AS2RAD * 1.0e-3
    dY = -0.0002259 * rastro.AS2RAD * 1.0e-3
    rastro.set_global_eop_from_static_values(pm_x, pm_y, ut1_utc, dX, dY, 0.0)

def test_bias_precession_nutation(static_eop):
    epc = rastro.Epoch.from_datetime(2007, 4, 5, 12, 0, 0, 0.0, "UTC")

    rc2i = rastro.bias_precession_nutation(epc)

    tol = 1e-8
    assert approx(rc2i[0, 0], +0.999999746339445, abs=tol)
    assert approx(rc2i[0, 1], -0.000000005138822, abs=tol)
    assert approx(rc2i[0, 2], -0.000712264730072, abs=tol)

    assert approx(rc2i[1, 0], -0.000000026475227, abs=tol)
    assert approx(rc2i[1, 1], +0.999999999014975, abs=tol)
    assert approx(rc2i[1, 2], -0.000044385242827, abs=tol)

    assert approx(rc2i[2, 0], +0.000712264729599, abs=tol)
    assert approx(rc2i[2, 1], +0.000044385250426, abs=tol)
    assert approx(rc2i[2, 2], +0.999999745354420, abs=tol)

def test_earth_rotation(static_eop):
    epc = rastro.Epoch.from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, "UTC")

    r = rastro.earth_rotation(epc) @ rastro.bias_precession_nutation(epc)

    tol = 1e-8
    assert approx(r[0, 0], +0.973104317573127, abs=tol)
    assert approx(r[0, 1], +0.230363826247709, abs=tol)
    assert approx(r[0, 2], -0.000703332818845, abs=tol)

    assert approx(r[1, 0], -0.230363798804182, abs=tol)
    assert approx(r[1, 1], +0.973104570735574, abs=tol)
    assert approx(r[1, 2], +0.000120888549586, abs=tol)

    assert approx(r[2, 0], +0.000712264729599, abs=tol)
    assert approx(r[2, 1], +0.000044385250426, abs=tol)
    assert approx(r[2, 2], +0.999999745354420, abs=tol)

def test_eci_to_ecef(static_eop):
    epc = rastro.Epoch.from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, "UTC")

    r = rastro.rotation_eci_to_ecef(epc)

    tol = 1e-8
    assert approx(r[0, 0], +0.973104317697535, abs=tol)
    assert approx(r[0, 1], +0.230363826239128, abs=tol)
    assert approx(r[0, 2], -0.000703163482198, abs=tol)

    assert approx(r[1, 0], -0.230363800456037, abs=tol)
    assert approx(r[1, 1], +0.973104570632801, abs=tol)
    assert approx(r[1, 2], +0.000118545366625, abs=tol)

    assert approx(r[2, 0], +0.000711560162668, abs=tol)
    assert approx(r[2, 1], +0.000046626403995, abs=tol)
    assert approx(r[2, 2], +0.999999745754024, abs=tol)

def test_ecef_to_eci(static_eop):
    epc = rastro.Epoch.from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, "UTC")

    r = rastro.rotation_ecef_to_eci(epc)

    tol = 1e-8
    assert approx(r[0, 0], +0.973104317697535, abs=tol)
    assert approx(r[0, 1], -0.230363800456037, abs=tol)
    assert approx(r[0, 2], +0.000711560162668, abs=tol)

    assert approx(r[1, 0], +0.230363826239128, abs=tol)
    assert approx(r[1, 1], +0.973104570632801, abs=tol)
    assert approx(r[1, 2], +0.000046626403995, abs=tol)

    assert approx(r[2, 0], -0.000703163482198, abs=tol)
    assert approx(r[2, 1], +0.000118545366625, abs=tol)
    assert approx(r[2, 2], +0.999999745754024, abs=tol)