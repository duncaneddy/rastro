import rastro

if __name__ == '__main__':
    # Example 1: Load Default C04 File with extrapolation

    # EOPExtrapolation is string that maps to the Rust extrapolation mode. The possible values are:
    #   - "Hold"
    #   - "Zero"
    #   - "Error"
    #
    # The Interpolation mode here is set as `True`
    eop = rastro.EarthOrientationData.from_default_c04("Hold", True)

    # Last ut1_utc offset stored in table.
    # eop.mjd_max is the maximum MJD date of data loaded in the table.
    last_ut1_utc = eop.get_ut1_utc(eop.mjd_max)

    # Get UT1_UTC value that is well beyond the end of the loaded data
    hold_ut1_utc = eop.get_ut1_utc(9999999.9)

    # Confirm that the EOP provider extrapolated beyond the end of the table by holding the value
    assert last_ut1_utc == hold_ut1_utc

    # Example 2: Load Default C04 data with "Zero" extrapolation value
    eop = rastro.EarthOrientationData.from_default_c04("Zero", True)

    # Confirm that values beyond the end of table are zero
    assert eop.get_ut1_utc(9999999.9) == 0.0

    # Example 3: Load C04 data from user-provided file

    # filepath = "~/PATH/TO/YOUR/EOP_FILE/iau2000A_c04_14.txt"
    # eop = EarthOrientationData::from_c04_file(filepath, "Error", False)