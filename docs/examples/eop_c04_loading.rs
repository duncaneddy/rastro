use rastro::eop::*;

fn main() {
    // Example 1: Load Default C04 File with extrapolation

    // EOPExtrapolation is a enum that sets the extrapolation mode. It has possible values of:
    //   - EOPExtrapolation::Hold
    //   - EOPExtrapolation::Zero
    //   - EOPExtrapolation::Error
    //
    // The Interpolation mode here is set as `true`
    set_global_eop_from_default_c04(EOPExtrapolation::Hold, true).unwrap();

    // Last ut1_utc offset stored in table.
    // eop.mjd_max is the maximum MJD date of data loaded in the table.
    let last_ut1_utc = get_global_ut1_utc(get_global_eop_mjd_max().into()).unwrap();

    // Get UT1_UTC value that is well beyond the end of the loaded data
    let hold_ut1_utc = get_global_ut1_utc(9999999.9).unwrap();

    // Confirm that the EOP provider extrapolated beyond the end of the table by holding the value
    assert!(last_ut1_utc == hold_ut1_utc);

    // Example 2: Load Default C04 data with "Zero" extrapolation value
    set_global_eop_from_default_c04(EOPExtrapolation::Zero, true).unwrap();

    // Confirm that values beyond the end of table are zero
    assert!(get_global_ut1_utc(9999999.9).unwrap() == 0.0);

    // Example 3: Load C04 data from user-provided file

    // let filepath = Path::new("~/PATH/TO/YOUR/EOP_FILE/iau2000A_c04_14.txt");
    // set_global_eop_from_c04_file(filepath, EOPExtrapolation::Error, false);
}
