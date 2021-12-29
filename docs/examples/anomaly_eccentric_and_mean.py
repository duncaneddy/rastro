import pytest
import rastro

if __name__ == '__main__':
    ecc = 45.0 # Starting eccentric anomaly
    e = 0.01  # Eccentricity

    # Convert to mean anomaly
    mean_anomaly = rastro.anomaly_eccentric_to_mean(ecc, e, True)

    # Convert back from mean to eccentric anomaly
    ecc_2 = rastro.anomaly_mean_to_eccentric(mean_anomaly, e, True)

    # Confirm equality to within tolerance
    assert ecc == pytest.approx(ecc_2, abs=1e-14)