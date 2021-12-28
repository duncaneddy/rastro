use rastro::orbits::{anomaly_true_to_eccentric, anomaly_eccentric_to_true};

fn main() {
    let nu = 45.0; // Starting true anomaly
    let e = 0.01;  // Eccentricity

    // Convert to eccentric anomaly
    let ecc_anomaly = anomaly_true_to_eccentric(nu, e, true);
    println!("Rust Hello World {}", ecc_anomaly);
}