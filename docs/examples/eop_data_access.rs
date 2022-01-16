use rastro::eop::{EarthOrientationData, EOPExtrapolation, EOPType};

#[allow(non_snake_case)]
#[allow(unused)]
fn main() {
    // Load Default C04 File with extrapolation
    let eop = EarthOrientationData::from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA);

    // Get UT1-UTC off set for a given MJD. Value is in seconds.
    let ut1_utc = eop.get_ut1_utc(59569.0);

    // Get x- and y-components of polar motion. Value is in radians.
    let (pm_x, pm_y) = eop.get_pm(59569.0);

    // Get dX and dY Precession/Nutation model corrections. Value is in radians.
    let (dX, dY) = eop.get_dxdy(59569.0);

    // Get Length of Day (LOD) offset. Value is in seconds.
    let lod = eop.get_lod(59569.0);
}
