import rastro

if __name__ == '__main__':
    # Load Default C04 File with extrapolation
    rastro.set_global_eop_from_default_standard("Hold", True, "StandardBulletinA")

    # Get UT1-UTC off set for a given MJD. Value is in seconds.
    ut1_utc = rastro.get_global_ut1_utc(59569.0)

    # Get x- and y-components of polar motion. Value is in radians.
    pm_x, pm_y = rastro.get_global_pm(59569.0)

    # Get dX and dY Precession/Nutation model corrections. Value is in radians.
    dX, dY = rastro.get_global_dxdy(59569.0)

    # Get Length of Day (LOD) offset. Value is in seconds.
    lod = rastro.get_global_lod(59569.0)