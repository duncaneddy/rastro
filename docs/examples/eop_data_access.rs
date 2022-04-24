use rastro::eop::*;

#[allow(non_snake_case)]
#[allow(unused)]
fn main() {
    // Load Default standard EOP data with extrapolation
    set_global_eop_from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA)
        .unwrap();

    // Get UT1-UTC off set for a given MJD. Value is in seconds.
    let ut1_utc = get_global_ut1_utc(59569.0).unwrap();

    // Get x- and y-components of polar motion. Value is in radians.
    let (pm_x, pm_y) = get_global_pm(59569.0).unwrap();

    // Get dX and dY Precession/Nutation model corrections. Value is in radians.
    let (dX, dY) = get_global_dxdy(59569.0).unwrap();

    // Get Length of Day (LOD) offset. Value is in seconds.
    let lod = get_global_lod(59569.0).unwrap();
}
