import rastro

if __name__ == '__main__':
    # Load Default C04 File with extrapolation
    eop = rastro.EarthOrientationData.from_default_standard("Hold", True, "StandardBulletinA")

    # Get UT1-UTC off set for a given MJD. Value is in seconds.
    ut1_utc = eop.get_ut1_utc(59569.0)

    # Get x- and y-components of polar motion. Value is in radians.
    pm_x, pm_y = eop.get_pm(59569.0)

    # Get dX and dY Precession/Nutation model corrections. Value is in radians.
    dX, dY = eop.get_dxdy(59569.0)

    # Get Length of Day (LOD) offset. Value is in seconds.
    lod = eop.get_lod(59569.0)