import numpy
import pytest
import rastro
import numpy as np
from pytest import approx

def test_state_osculating_to_cartesian(eop):
    osc = numpy.array([rastro.R_EARTH + 500e3, 0.0, 0.0, 0.0, 0.0, 0.0])
    cart = rastro.state_osculating_to_cartesian(osc, False)

    assert isinstance(cart, np.ndarray)
    assert cart[0] == rastro.R_EARTH + 500e3
    assert cart[1] == 0.0
    assert cart[2] == 0.0
    assert cart[3] == 0.0
    assert cart[4] == rastro.perigee_velocity(rastro.R_EARTH + 500e3, 0.0)
    assert cart[5] == 0.0

    osc = numpy.array([rastro.R_EARTH + 500e3, 0.0, 90.0, 0.0, 0.0, 0.0])
    cart = rastro.state_osculating_to_cartesian(osc, True)

    assert isinstance(cart, np.ndarray)
    assert cart[0] == rastro.R_EARTH + 500e3
    assert cart[1] == 0.0
    assert cart[2] == 0.0
    assert cart[3] == 0.0
    assert cart[4] == pytest.approx(0.0, abs=1.0e-12)
    assert cart[5] == rastro.perigee_velocity(rastro.R_EARTH + 500e3, 0.0)

def test_state_cartesian_to_osculating(eop):
    cart = np.array([
        rastro.R_EARTH + 500e3,
        0.0,
        0.0,
        0.0,
        rastro.perigee_velocity(rastro.R_EARTH + 500e3, 0.0),
        0.0,
    ])
    osc = rastro.state_cartesian_to_osculating(cart, True)

    assert osc[0] == approx(rastro.R_EARTH + 500e3, abs = 1e-9)
    assert osc[1] == 0.0
    assert osc[2] == 0.0
    assert osc[3] == 180.0
    assert osc[4] == 0.0
    assert osc[5] == 0.0

    cart = np.array([
        rastro.R_EARTH + 500e3,
        0.0,
        0.0,
        0.0,
        0.0,
        rastro.perigee_velocity(rastro.R_EARTH + 500e3, 0.0),
    ])
    osc = rastro.state_cartesian_to_osculating(cart, True)

    assert osc[0] == approx(rastro.R_EARTH + 500e3, abs = 1.0e-9)
    assert osc[1] == 0.0
    assert osc[2] == 90.0
    assert osc[3] == 0.0
    assert osc[4] == 0.0
    assert osc[5] == 0.0

def test_position_eci_to_ecef(eop):
    epc = rastro.Epoch.from_datetime(2022, 4, 5, 0, 0, 0.0, 0.0, "UTC")

    p_eci = np.array([rastro.R_EARTH + 500e3, 0.0, 0.0])

    p_ecef = rastro.position_eci_to_ecef(epc, p_eci)

    assert p_eci[0] != p_ecef[0]
    assert p_eci[1] != p_ecef[1]
    assert p_eci[2] != p_ecef[2]

def test_position_ecef_to_eci(eop):
    epc = rastro.Epoch.from_datetime(2022, 4, 5, 0, 0, 0.0, 0.0, "UTC")

    p_ecef = np.array([rastro.R_EARTH + 500e3, 0.0, 0.0])

    p_eci = rastro.position_ecef_to_eci(epc, p_ecef)

    assert p_eci[0] != p_ecef[0]
    assert p_eci[1] != p_ecef[1]
    assert p_eci[2] != p_ecef[2]

def test_state_eci_to_ecef_circular(eop):
    epc = rastro.Epoch.from_datetime(2022, 4, 5, 0, 0, 0.0, 0.0, "UTC")

    oe = np.array([rastro.R_EARTH + 500e3, 1e-3, 97.8, 75.0, 25.0, 45.0])
    eci = rastro.state_osculating_to_cartesian(oe, True)

    # Perform circular transformations
    ecef = rastro.state_eci_to_ecef(epc, eci)
    eci2 = rastro.state_ecef_to_eci(epc, ecef)
    ecef2 = rastro.state_eci_to_ecef(epc, eci2)

    tol = 1e-6
    # Check equivalence of ECI transforms
    assert eci2[0] == approx(eci[0], abs=tol)
    assert eci2[1] == approx(eci[1], abs=tol)
    assert eci2[2] == approx(eci[2], abs=tol)
    assert eci2[3] == approx(eci[3], abs=tol)
    assert eci2[4] == approx(eci[4], abs=tol)
    assert eci2[5] == approx(eci[5], abs=tol)
    # Check equivalence of ECEF transforms
    assert ecef2[0] == approx(ecef[0], abs=tol)
    assert ecef2[1] == approx(ecef[1], abs=tol)
    assert ecef2[2] == approx(ecef[2], abs=tol)
    assert ecef2[3] == approx(ecef[3], abs=tol)
    assert ecef2[4] == approx(ecef[4], abs=tol)
    assert ecef2[5] == approx(ecef[5], abs=tol)

def test_position_geocentric_to_ecef(eop):
    pass

def test_position_ecef_to_geocentric(eop):
    pass

def test_position_geodetic_to_ecef(eop):
    pass

def test_position_ecef_to_geodetic(eop):
    pass
