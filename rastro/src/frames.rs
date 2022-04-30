use nalgebra as na;
use rsofa;

use crate::constants::MJD_ZERO;
use crate::eop;
use crate::time::{Epoch, TimeSystem};

fn matrix3_from_array(mat: &[[f64; 3]; 3]) -> na::Matrix3<f64> {
    na::Matrix3::new(
        mat[0][0], mat[0][1], mat[0][2], mat[1][0], mat[1][1], mat[1][2], mat[2][0], mat[2][1],
        mat[2][2],
    )
}

/// Computes the Bias-Precession-Nutation matrix transforming the GCRS to the
/// CIRS intermediate reference frame. This transformation corrects for the
/// bias, precession, and nutation of Celestial Intermediate Origin (CIO) with
/// respect to inertial space.
///
/// This formulation computes the Bias-Precession-Nutation correction matrix
/// according using a CIO based model using using the IAU 2006
/// precession and IAU 2000A nutation models.
///
/// The function will utilize the global Earth orientation and loaded data to
/// apply corrections to the Celestial Intermediate Pole (CIP) derived from
/// empirical observations.
///
/// # Arguments:
/// - `epc`: Epoch instant for computation of transformation matrix
///
/// # Returns:
/// - `rc2i`: 3x3 Rotation matrix transforming GCRS -> CIRS
///
/// # Examples:
/// ```rust
/// use rastro::eop::{set_global_eop_from_default_standard, EOPExtrapolation, EOPType};
/// use rastro::time::{Epoch, TimeSystem};
/// use rastro::frames::*;
///
/// // Quick EOP initialization
/// set_global_eop_from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA).unwrap();
///
/// let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);
///
/// let rc2i = bias_precession_nutation(epc);
/// ```
///
/// # References:
/// - [IAU SOFA Tools For Earth Attitude, Example 5.5](http://www.iausofa.org/2021_0512_C/sofa/sofa_pn_c.pdf) Software Version 18, 2021-04-18
#[allow(non_snake_case)]
pub fn bias_precession_nutation(epc: Epoch) -> na::Matrix3<f64> {
    // Compute X, Y, s terms using low-precision series terms
    let mut x = 0.0;
    let mut y = 0.0;
    let mut s = 0.0;

    unsafe {
        rsofa::iauXys06a(
            MJD_ZERO,
            epc.mjd_as_tsys(TimeSystem::TT),
            &mut x,
            &mut y,
            &mut s,
        );
    }

    // Apply Celestial Intermediate Pole corrections
    let (dX, dY) = eop::get_global_dxdy(epc.mjd_as_tsys(TimeSystem::UTC)).unwrap();
    x += dX;
    y += dY;

    // Compute transformation
    let mut rc2i = [[0.0; 3]; 3];
    unsafe {
        rsofa::iauC2ixys(x, y, s, &mut rc2i[0]);
    }

    matrix3_from_array(&rc2i)
}

/// Computes the Earth rotation matrix transforming the CIRS to the TIRS
/// intermediate reference frame. This transformation corrects for the Earth
/// rotation.
///
/// # Arguments:
/// - `epc`: Epoch instant for computation of transformation matrix
///
/// # Returns:
/// - `r`: 3x3 Rotation matrix transforming CIRS -> TIRS
///
/// # Examples:
/// ```rust
/// use rastro::eop::{set_global_eop_from_default_standard, EOPExtrapolation, EOPType};
/// use rastro::time::{Epoch, TimeSystem};
/// use rastro::frames::*;
///
/// // Quick EOP initialization
/// set_global_eop_from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA).unwrap();
///
/// let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);
///
/// let r = earth_rotation(epc);
/// ```
///
/// # References:
/// - [IAU SOFA  Tools For Earth Attitude, Example 5.5](http://www.iausofa.org/2021_0512_C/sofa/sofa_pn_c.pdf) Software Version 18, 2021-04-18
pub fn earth_rotation(epc: Epoch) -> na::Matrix3<f64> {
    let mut r = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    unsafe {
        // Compute Earth rotation angle
        let era = rsofa::iauEra00(MJD_ZERO, epc.mjd_as_tsys(TimeSystem::UT1));

        // Construct Earth-rotation rotation matrix
        rsofa::iauRz(era, &mut r[0]);
    }

    matrix3_from_array(&r)
}

/// Computes the Earth rotation matrix transforming the TIRS to the ITRF reference
/// frame.
///
/// The function will utilize the global Earth orientation and loaded data to
/// apply corrections to compute the polar motion correction based on empirical
/// observations of polar motion drift.
///
/// # Arguments:
/// - `epc`: Epoch instant for computation of transformation matrix
///
/// # Returns:
/// - `rpm`: 3x3 Rotation matrix transforming TIRS -> ITRF
///
/// # Examples:
/// ```rust
/// use rastro::eop::{set_global_eop_from_default_standard, EOPExtrapolation, EOPType};
/// use rastro::time::{Epoch, TimeSystem};
/// use rastro::frames::*;
///
/// // Quick EOP initialization
/// set_global_eop_from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA).unwrap();
///
/// let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);
///
/// let r = polar_motion(epc);
/// ```
///
/// # References:
/// - [IAU SOFA  Tools For Earth Attitude, Example 5.5](http://www.iausofa.org/2021_0512_C/sofa/sofa_pn_c.pdf) Software Version 18, 2021-04-18
pub fn polar_motion(epc: Epoch) -> na::Matrix3<f64> {
    let mut rpm = [[0.0; 3]; 3];

    let (pm_x, pm_y) = eop::get_global_pm(epc.mjd_as_tsys(TimeSystem::TT)).unwrap();

    unsafe {
        rsofa::iauPom00(
            pm_x,
            pm_y,
            rsofa::iauSp00(MJD_ZERO, epc.mjd_as_tsys(TimeSystem::TT)),
            &mut rpm[0],
        );
    }

    matrix3_from_array(&rpm)
}

/// Computes the combined rotation matrix from the inertial to the Earth-fixed
/// reference frame. Applies corrections for bias, precession, nutation,
/// Earth-rotation, and polar motion.
///
/// The transformation is accomplished using the IAU 2006/2000A, CIO-based
/// theory using classical angles. The method as described in section 5.5 of
/// the SOFA C transformation cookbook.
///
/// The function will utilize the global Earth orientation and loaded data to
/// apply corrections for Celestial Intermidate Pole (CIP) and polar motion drift
/// derived from empirical observations.
///
/// # Arguments:
/// - `epc`: Epoch instant for computation of transformation matrix
///
/// # Returns:
/// - `r`: 3x3 Rotation matrix transforming GCRF -> ITRF
///
/// # Examples:
/// ```rust
/// use rastro::eop::{set_global_eop_from_default_standard, EOPExtrapolation, EOPType};
/// use rastro::time::{Epoch, TimeSystem};
/// use rastro::frames::*;
///
/// // Quick EOP initialization
/// set_global_eop_from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA).unwrap();
///
/// let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);
///
/// let r = rotation_eci_to_ecef(epc);
/// ```
///
/// # References:
/// - [IAU SOFA  Tools For Earth Attitude, Example 5.5](http://www.iausofa.org/2021_0512_C/sofa/sofa_pn_c.pdf) Software Version 18, 2021-04-18
pub fn rotation_eci_to_ecef(epc: Epoch) -> na::Matrix3<f64> {
    polar_motion(epc) * earth_rotation(epc) * bias_precession_nutation(epc)
}

// pub fn sECItoECEF(epc:Epoch, x:na::Vector3<f64>) -> na::Vector3<f64>:
/// Computes the combined rotation matrix from the Earth-fixed to the inertial
/// reference frame. Applies corrections for bias, precession, nutation,
/// Earth-rotation, and polar motion.
///
/// The transformation is accomplished using the IAU 2006/2000A, CIO-based
/// theory using classical angles. The method as described in section 5.5 of
/// the SOFA C transformation cookbook.
///
/// The function will utilize the global Earth orientation and loaded data to
/// apply corrections for Celestial Intermidate Pole (CIP) and polar motion drift
/// derived from empirical observations.
///
/// # Arguments:
/// - `epc`: Epoch instant for computation of transformation matrix
///
/// # Returns:
/// - `r`: 3x3 Rotation matrix transforming ITRF -> GCRF
///
/// # Examples:
/// ```rust
/// use rastro::eop::{set_global_eop_from_default_standard, EOPExtrapolation, EOPType};
/// use rastro::time::{Epoch, TimeSystem};
/// use rastro::frames::*;
///
/// // Quick EOP initialization
/// set_global_eop_from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA).unwrap();
///
/// let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);
///
/// let r = rotation_ecef_to_eci(epc);
/// ```
///
/// # References:
/// - [IAU SOFA  Tools For Earth Attitude, Example 5.5](http://www.iausofa.org/2021_0512_C/sofa/sofa_pn_c.pdf) Software Version 18, 2021-04-18
pub fn rotation_ecef_to_eci(epc: Epoch) -> na::Matrix3<f64> {
    rotation_eci_to_ecef(epc).transpose()
}
// pub fn sECEFtoECI(epc:Epoch, x:na::Vector3<f64>) -> na::Vector3<f64>:

#[cfg(test)]
mod tests {
    use crate::constants::AS2RAD;
    use crate::eop::*;
    use crate::frames::*;
    use approx::assert_abs_diff_eq;

    #[allow(non_snake_case)]
    fn set_test_static_eop() {
        // Constants of IAU 2006A transformation
        let pm_x = 0.0349282 * AS2RAD;
        let pm_y = 0.4833163 * AS2RAD;
        let ut1_utc = -0.072073685;
        let dX = 0.0001750 * AS2RAD * 1.0e-3;
        let dY = -0.0002259 * AS2RAD * 1.0e-3;
        set_global_eop_from_static_values(pm_x, pm_y, ut1_utc, dX, dY, 0.0);
        // assert_eq!(GLOBAL_EOP.initialized(), true);
    }

    #[test]
    fn test_matrix3_from_array() {
        let mat = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];

        let na_mat = matrix3_from_array(&mat);

        assert_eq!(na_mat[(0, 0)], 1.0);
        assert_eq!(na_mat[(0, 1)], 2.0);
        assert_eq!(na_mat[(0, 2)], 3.0);

        assert_eq!(na_mat[(1, 0)], 4.0);
        assert_eq!(na_mat[(1, 1)], 5.0);
        assert_eq!(na_mat[(1, 2)], 6.0);

        assert_eq!(na_mat[(2, 0)], 7.0);
        assert_eq!(na_mat[(2, 1)], 8.0);
        assert_eq!(na_mat[(2, 2)], 9.0);
    }

    #[test]
    fn test_bias_precession_nutation() {
        // Test case reproduction of Example 5.5 from SOFA cookbook

        // Set Earth orientation parameters for test case
        set_test_static_eop();

        // Set Epoch
        let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);

        let rc2i = bias_precession_nutation(epc);

        let tol = 1.0e-8;
        assert_abs_diff_eq!(rc2i[(0, 0)], 0.999999746339445, epsilon = tol);
        assert_abs_diff_eq!(rc2i[(0, 1)], -0.000000005138822, epsilon = tol);
        assert_abs_diff_eq!(rc2i[(0, 2)], -0.000712264730072, epsilon = tol);

        assert_abs_diff_eq!(rc2i[(1, 0)], -0.000000026475227, epsilon = tol);
        assert_abs_diff_eq!(rc2i[(1, 1)], 0.999999999014975, epsilon = tol);
        assert_abs_diff_eq!(rc2i[(1, 2)], -0.000044385242827, epsilon = tol);

        assert_abs_diff_eq!(rc2i[(2, 0)], 0.000712264729599, epsilon = tol);
        assert_abs_diff_eq!(rc2i[(2, 1)], 0.000044385250426, epsilon = tol);
        assert_abs_diff_eq!(rc2i[(2, 2)], 0.999999745354420, epsilon = tol);
    }

    #[test]
    fn test_earth_rotation() {
        // Test case reproduction of Example 5.5 from SOFA cookbook

        // Set Earth orientation parameters for test case
        set_test_static_eop();

        // Set Epoch
        let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);

        let r = earth_rotation(epc) * bias_precession_nutation(epc);

        let tol = 1.0e-8;
        assert_abs_diff_eq!(r[(0, 0)], 0.973104317573127, epsilon = tol);
        assert_abs_diff_eq!(r[(0, 1)], 0.230363826247709, epsilon = tol);
        assert_abs_diff_eq!(r[(0, 2)], -0.000703332818845, epsilon = tol);

        assert_abs_diff_eq!(r[(1, 0)], -0.230363798804182, epsilon = tol);
        assert_abs_diff_eq!(r[(1, 1)], 0.973104570735574, epsilon = tol);
        assert_abs_diff_eq!(r[(1, 2)], 0.000120888549586, epsilon = tol);

        assert_abs_diff_eq!(r[(2, 0)], 0.000712264729599, epsilon = tol);
        assert_abs_diff_eq!(r[(2, 1)], 0.000044385250426, epsilon = tol);
        assert_abs_diff_eq!(r[(2, 2)], 0.999999745354420, epsilon = tol);
    }

    #[test]
    fn test_rotation_eci_to_ecef() {
        // Test case reproduction of Example 5.5 from SOFA cookbook

        // Set Earth orientation parameters for test case
        set_test_static_eop();

        // Set Epoch
        let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);

        let r = rotation_eci_to_ecef(epc);

        let tol = 1.0e-8;
        assert_abs_diff_eq!(r[(0, 0)], 0.973104317697535, epsilon = tol);
        assert_abs_diff_eq!(r[(0, 1)], 0.230363826239128, epsilon = tol);
        assert_abs_diff_eq!(r[(0, 2)], -0.000703163482198, epsilon = tol);

        assert_abs_diff_eq!(r[(1, 0)], -0.230363800456037, epsilon = tol);
        assert_abs_diff_eq!(r[(1, 1)], 0.973104570632801, epsilon = tol);
        assert_abs_diff_eq!(r[(1, 2)], 0.000118545366625, epsilon = tol);

        assert_abs_diff_eq!(r[(2, 0)], 0.000711560162668, epsilon = tol);
        assert_abs_diff_eq!(r[(2, 1)], 0.000046626403995, epsilon = tol);
        assert_abs_diff_eq!(r[(2, 2)], 0.999999745754024, epsilon = tol);
    }

    #[test]
    fn test_rotation_ecef_to_eci() {
        // Test case reproduction of Example 5.5 from SOFA cookbook

        // Set Earth orientation parameters for test case
        set_test_static_eop();

        // Set Epoch
        let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);

        let r = rotation_ecef_to_eci(epc);

        let tol = 1.0e-8;
        assert_abs_diff_eq!(r[(0, 0)], 0.973104317697535, epsilon = tol);
        assert_abs_diff_eq!(r[(0, 1)], -0.230363800456037, epsilon = tol);
        assert_abs_diff_eq!(r[(0, 2)], 0.000711560162668, epsilon = tol);

        assert_abs_diff_eq!(r[(1, 0)], 0.230363826239128, epsilon = tol);
        assert_abs_diff_eq!(r[(1, 1)], 0.973104570632801, epsilon = tol);
        assert_abs_diff_eq!(r[(1, 2)], 0.000046626403995, epsilon = tol);

        assert_abs_diff_eq!(r[(2, 0)], -0.000703163482198, epsilon = tol);
        assert_abs_diff_eq!(r[(2, 1)], 0.000118545366625, epsilon = tol);
        assert_abs_diff_eq!(r[(2, 2)], 0.999999745754024, epsilon = tol);
    }
}
