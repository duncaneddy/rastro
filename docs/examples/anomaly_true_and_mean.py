import pytest
import rastro

if __name__ == '__main__':
    nu = 45.0 # Starting true anomaly
    e = 0.01  # Eccentricity

    # Convert to mean anomaly
    mean_anomaly = rastro.anomaly_true_to_mean(nu, e, True)

    # Convert back from eccentric to true anomaly
    nu_2 = rastro.anomaly_mean_to_true(mean_anomaly, e, True)

    # Confirm equality to within tolerance
    assert nu == pytest.approx(nu_2, abs=1e-14)