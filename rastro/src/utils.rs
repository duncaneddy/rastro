use crate::constants;
use nalgebra as na;

pub fn from_degrees(num: f64, as_degrees: bool) -> f64 {
    if as_degrees {
        num * constants::DEG2RAD
    } else {
        num
    }
}

pub fn to_degrees(num: f64, as_degrees: bool) -> f64 {
    if as_degrees {
        num * constants::RAD2DEG
    } else {
        num
    }
}

pub fn vector3_from_array(vec: [f64; 3]) -> na::Vector3<f64> {
    na::Vector3::new(vec[0], vec[1], vec[2])
}

pub fn vector6_from_array(vec: [f64; 6]) -> na::Vector6<f64> {
    na::Vector6::new(vec[0], vec[1], vec[2], vec[3], vec[4], vec[5])
}

pub fn matrix3_from_array(mat: &[[f64; 3]; 3]) -> na::Matrix3<f64> {
    na::Matrix3::new(
        mat[0][0], mat[0][1], mat[0][2], mat[1][0], mat[1][1], mat[1][2], mat[2][0], mat[2][1],
        mat[2][2],
    )
}
