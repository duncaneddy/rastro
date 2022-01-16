use rastro::eop::{EarthOrientationData, EOPExtrapolation, EOPType};

fn main() {
    // Example 1: Load Default C04 File with extrapolation

    // EOPExtrapolation is a enum that sets the extrapolation mode. It has possible values of:
    //   - EOPExtrapolation::Hold
    //   - EOPExtrapolation::Zero
    //   - EOPExtrapolation::Error
    //
    // The Interpolation mode here is set as `true`
    //
    // EOPType is an enum that sets indicates which type of Earth Orientation to load from a file.
    // It is also a property of all EarthOrientationData objects that provides what type of data
    // was loaded into the object. Possible values are:
    //   - EOPType::StandardBulletinA
    //   - EOPType::StandardBulletinB
    //   - EOPType::C04
    let eop = EarthOrientationData::from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA);

    // Last ut1_utc offset stored in table.
    // eop.mjd_max is the maximum MJD date of data loaded in the table.
    let last_ut1_utc = eop.get_ut1_utc(eop.mjd_max.into());

    // Get UT1_UTC value that is well beyond the end of the loaded data
    let hold_ut1_utc = eop.get_ut1_utc(9999999.9);

    // Confirm that the EOP provider extrapolated beyond the end of the table by holding the value
    assert!(last_ut1_utc == hold_ut1_utc);


    // Example 2: Load Default C04 data with "Zero" extrapolation value
    let eop = EarthOrientationData::from_default_standard(EOPExtrapolation::Zero, true, EOPType::StandardBulletinB);

    // Confirm that values beyond the end of table are zero
    assert!(eop.get_ut1_utc(9999999.9) == 0.0);

    // Example 3: Load Standard data from user-provided file

    // let filepath = Path::new("~/PATH/TO/YOUR/EOP_FILE/iau2000A_finals_ab.txt");
    // let eop = EarthOrientationData::from_standard_file(filepath, EOPExtrapolation::Error, false, EOPType::StandardBulletinB);
}
