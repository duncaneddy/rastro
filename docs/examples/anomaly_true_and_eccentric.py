import pytest
import rastro

if __name__ == '__main__':
    nu = 45.0; # Starting true anomaly
    e = 0.01;  # Eccentricity

    # Convert to eccentric anomaly
    ecc_anomaly = rastro.anomaly_true_to_eccentric(nu, e, True)

    # Convert back from eccentric to true anomaly
    nu_2 = rastro.anomaly_eccentric_to_true(ecc_anomaly, e, True)

    # Confirm equality to within tolerance
    assert nu == pytest.approx(nu_2, abs=1e-14)