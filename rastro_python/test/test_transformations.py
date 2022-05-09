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

def test_position_geocentric(eop):
    tol = 1.0e-7

    # Test known position conversions
    geoc1 = np.array([0.0, 0.0, 0.0])
    ecef1 = rastro.position_geocentric_to_ecef(geoc1, True)

    assert ecef1[0] == approx(rastro.WGS84_A, abs=tol)
    assert ecef1[1] == approx(0.0, abs=tol)
    assert ecef1[2] == approx(0.0, abs=tol)

    geoc2 = np.array([90.0, 0.0, 0.0])
    ecef2 = rastro.position_geocentric_to_ecef(geoc2, True)

    assert ecef2[0] == approx(0.0, abs=tol)
    assert ecef2[1] == approx(rastro.WGS84_A, abs=tol)
    assert ecef2[2] == approx(0.0, abs=tol)

    geoc3 = np.array([0.0, 90.0, 0.0])
    ecef3 = rastro.position_geocentric_to_ecef(geoc3, True)

    assert ecef3[0] == approx(0.0, abs=tol)
    assert ecef3[1] == approx(0.0, abs=tol)
    assert ecef3[2] == approx(rastro.WGS84_A, abs=tol)

    # Test two-input format
    geoc = np.array([0.0, 0.0, 0.0])
    ecef = rastro.position_geocentric_to_ecef(geoc, True)

    assert ecef[0] == approx(rastro.WGS84_A, abs=tol)
    assert ecef[1] == approx(0.0, abs=tol)
    assert ecef[2] == approx(0.0, abs=tol)

    geoc = np.array([90.0, 0.0, 0.0])
    ecef = rastro.position_geocentric_to_ecef(geoc, True)

    assert ecef[0] == approx(0.0, abs=tol)
    assert ecef[1] == approx(rastro.WGS84_A, abs=tol)
    assert ecef[2] == approx(0.0, abs=tol)

    geoc = np.array([0.0, 90.0, 0.0])
    ecef = rastro.position_geocentric_to_ecef(geoc, True)

    assert ecef[0] == approx(0.0, abs=tol)
    assert ecef[1] == approx(0.0, abs=tol)
    assert ecef[2] == approx(rastro.WGS84_A, abs=tol)

    # Test circularity
    geoc4 = rastro.position_ecef_to_geocentric(ecef1, True)
    geoc5 = rastro.position_ecef_to_geocentric(ecef2, True)
    geoc6 = rastro.position_ecef_to_geocentric(ecef3, True)

    assert geoc4[0] == approx(geoc1[0], abs=tol)
    assert geoc4[1] == approx(geoc1[1], abs=tol)
    assert geoc4[2] == approx(geoc1[2], abs=tol)

    assert geoc5[0] == approx(geoc2[0], abs=tol)
    assert geoc5[1] == approx(geoc2[1], abs=tol)
    assert geoc5[2] == approx(geoc2[2], abs=tol)

    assert geoc6[0] == approx(geoc3[0], abs=tol)
    assert geoc6[1] == approx(geoc3[1], abs=tol)
    assert geoc6[2] == approx(geoc3[2], abs=tol)

    # Random point circularity
    geoc  = np.array([77.875000, 20.975200, 0.000000])
    ecef  = rastro.position_geocentric_to_ecef(geoc, True)
    geocc = rastro.position_ecef_to_geocentric(ecef, True)
    assert geoc[0] == approx(geocc[0], abs=tol)
    assert geoc[1] == approx(geocc[1], abs=tol)
    assert geoc[2] == approx(geocc[2], abs=tol)

@pytest.mark.xfail()
@pytest.mark.parametrize("lat", [90.1, -90.1])
def test_geocentric_failure(eop, lat):
    # Test Error Condition
    with pytest.raises(RuntimeError):
        rastro.position_geocentric_to_ecef(np.array([0.0,  lat, 0.0]), True)

def test_position_geodetic(eop):
    tol = 1.0e-7

    # Test known position conversions
    geod1 = np.array([0.0, 0.0, 0.0])
    ecef1 = rastro.position_geodetic_to_ecef(geod1, True)

    assert ecef1[0] == approx(rastro.WGS84_A, abs=tol)
    assert ecef1[1] == approx(0.0, abs=tol)
    assert ecef1[2] == approx(0.0, abs=tol)

    geod2 = np.array([90.0, 0.0, 0.0])
    ecef2 = rastro.position_geodetic_to_ecef(geod2, True)

    assert ecef2[0] == approx(0.0, abs=tol)
    assert ecef2[1] == approx(rastro.WGS84_A, abs=tol)
    assert ecef2[2] == approx(0.0, abs=tol)

    geod3 = np.array([0.0, 90.0, 0.0])
    ecef3 = rastro.position_geodetic_to_ecef(geod3, True)

    assert ecef3[0] == approx(0.0, abs=tol)
    assert ecef3[1] == approx(0.0, abs=tol)
    assert ecef3[2] == approx(rastro.WGS84_A*(1.0-rastro.WGS84_F), abs=tol)

    # Test two input format
    geod = np.array([0.0, 0.0, 0.0])
    ecef = rastro.position_geodetic_to_ecef(geod, True)

    assert ecef[0] == approx(rastro.WGS84_A, abs=tol)
    assert ecef[1] == approx(0.0, abs=tol)
    assert ecef[2] == approx(0.0, abs=tol)

    geod = np.array([90.0, 0.0, 0.0])
    ecef = rastro.position_geodetic_to_ecef(geod, True)

    assert ecef[0] == approx(0.0, abs=tol)
    assert ecef[1] == approx(rastro.WGS84_A, abs=tol)
    assert ecef[2] == approx(0.0, abs=tol)

    geod = np.array([0.0, 90.0, 0.0])
    ecef = rastro.position_geodetic_to_ecef(geod, True)

    assert ecef[0] == approx(0.0, abs=tol)
    assert ecef[1] == approx(0.0, abs=tol)
    assert ecef[2] == approx(rastro.WGS84_A*(1.0-rastro.WGS84_F), abs=tol)

    # Test circularity
    geod4 = rastro.position_ecef_to_geodetic(ecef1, True)
    geod5 = rastro.position_ecef_to_geodetic(ecef2, True)
    geod6 = rastro.position_ecef_to_geodetic(ecef3, True)

    assert geod4[0] == approx(geod1[0], abs=tol)
    assert geod4[1] == approx(geod1[1], abs=tol)
    assert geod4[2] == approx(geod1[2], abs=tol)

    assert geod5[0] == approx(geod2[0], abs=tol)
    assert geod5[1] == approx(geod2[1], abs=tol)
    assert geod5[2] == approx(geod2[2], abs=tol)

    assert geod6[0] == approx(geod3[0], abs=tol)
    assert geod6[1] == approx(geod3[1], abs=tol)
    assert geod6[2] == approx(geod3[2], abs=tol)

    geod  = np.array([77.875000,    20.975200,     0.000000])
    ecef  = rastro.position_geodetic_to_ecef(geod, True)
    geodc = rastro.position_ecef_to_geodetic(ecef, True)
    assert geod[0] == approx(geodc[0], abs=tol)
    assert geod[1] == approx(geodc[1], abs=tol)
    assert geod[2] == approx(geodc[2], abs=tol)


@pytest.mark.xfail()
@pytest.mark.parametrize("lat", [90.1, -90.1])
def test_geodetic_failure(eop, lat):
    # Test Error Condition
    rastro.position_geodetic_to_ecef(np.array([0.0, lat, 0.0]), True)


