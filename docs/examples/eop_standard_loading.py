import rastro

if __name__ == '__main__':
    # Example 1: Load Default C04 File with extrapolation

    # EOPExtrapolation is string that maps to the Rust extrapolation mode. The possible values are:
    #   - "Hold"
    #   - "Zero"
    #   - "Error"
    #
    # The Interpolation mode here is set as `True`
    #
    # EOPType is an enum that sets indicates which type of Earth Orientation to load from a file.
    # It is also a property of all EarthOrientationData objects that provides what type of data
    # was loaded into the object. Possible values are:
    #   - "StandardBulletinA"
    #   - "StandardBulletinB"
    #   - "C04"
    rastro.set_global_eop_from_default_standard("Hold", True, "StandardBulletinA")

    # Last ut1_utc offset stored in table.
    # eop.mjd_max is the maximum MJD date of data loaded in the table.
    last_ut1_utc = rastro.get_global_ut1_utc(rastro.get_global_eop_mjd_max())

    # Get UT1_UTC value that is well beyond the end of the loaded data
    hold_ut1_utc = rastro.get_global_ut1_utc(9999999.9)

    # Confirm that the EOP provider extrapolated beyond the end of the table by holding the value
    assert last_ut1_utc == hold_ut1_utc

    # Example 2: Load Default C04 data with "Zero" extrapolation value
    rastro.set_global_eop_from_default_standard("Zero", True, "StandardBulletinA")

    # Confirm that values beyond the end of table are zero
    assert rastro.get_global_ut1_utc(9999999.9) == 0.0

    # Example 3: Load Standard data from user-provided file

    # filepath = "~/PATH/TO/YOUR/EOP_FILE/iau2000A_finals_ab.txt"
    # eop = EarthOrientationData::from_standard_file(filepath, "Error", False, "StandardBulletinA")