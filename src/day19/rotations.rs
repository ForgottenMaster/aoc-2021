use {crate::common::math::Matrix, std::collections::HashSet};

// sin constants
const SIN_0_DEG: i64 = 0;
const SIN_90_DEG: i64 = 1;
const SIN_180_DEG: i64 = 0;
const SIN_270_DEG: i64 = -1;

// cos constants
const COS_0_DEG: i64 = 1;
const COS_90_DEG: i64 = 0;
const COS_180_DEG: i64 = -1;
const COS_270_DEG: i64 = 0;

// Enum to identify angle of rotation around one axis
#[derive(Clone, Copy)]
enum Angle {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

/// Maps the angle onto a given sin value
const fn sin(angle: Angle) -> i64 {
    match angle {
        Angle::Zero => SIN_0_DEG,
        Angle::Ninety => SIN_90_DEG,
        Angle::OneEighty => SIN_180_DEG,
        Angle::TwoSeventy => SIN_270_DEG,
    }
}

/// Maps the angle onto a given cos value
const fn cos(angle: Angle) -> i64 {
    match angle {
        Angle::Zero => COS_0_DEG,
        Angle::Ninety => COS_90_DEG,
        Angle::OneEighty => COS_180_DEG,
        Angle::TwoSeventy => COS_270_DEG,
    }
}

/// Const function that produces a rotation matrix given the desired angles for rotation
/// around x, y, and z axis. Applies the rotations in the order of x first, then y, then z.
const fn rotation_matrix(x: Angle, y: Angle, z: Angle) -> Matrix<i64, 3, 3> {
    let (sin_y, sin_z, sin_x, cos_y, cos_z, cos_x) =
        (sin(y), sin(z), sin(x), cos(y), cos(z), cos(x));
    Matrix::new([
        [
            cos_z * cos_y,
            cos_z * sin_y * sin_x - sin_z * cos_x,
            cos_z * sin_y * cos_x + sin_z * sin_x,
        ],
        [
            sin_z * cos_y,
            sin_z * sin_y * sin_x + cos_z * cos_x,
            sin_z * sin_y * cos_x - cos_z * sin_x,
        ],
        [-sin_y, cos_y * sin_x, cos_y * cos_x],
    ])
}

// all the valid orientations for the scanners.
pub const ROTATIONS: [Matrix<i64, 3, 3>; 24] = [
    rotation_matrix(Angle::Zero, Angle::Ninety, Angle::OneEighty),
    rotation_matrix(Angle::Zero, Angle::Ninety, Angle::TwoSeventy),
    rotation_matrix(Angle::Zero, Angle::OneEighty, Angle::Ninety),
    rotation_matrix(Angle::Zero, Angle::OneEighty, Angle::TwoSeventy),
    rotation_matrix(Angle::Zero, Angle::TwoSeventy, Angle::Ninety),
    rotation_matrix(Angle::Zero, Angle::TwoSeventy, Angle::OneEighty),
    rotation_matrix(Angle::Ninety, Angle::Zero, Angle::OneEighty),
    rotation_matrix(Angle::Ninety, Angle::Zero, Angle::TwoSeventy),
    rotation_matrix(Angle::Ninety, Angle::OneEighty, Angle::Zero),
    rotation_matrix(Angle::Ninety, Angle::OneEighty, Angle::TwoSeventy),
    rotation_matrix(Angle::Ninety, Angle::TwoSeventy, Angle::Zero),
    rotation_matrix(Angle::Ninety, Angle::TwoSeventy, Angle::OneEighty),
    rotation_matrix(Angle::OneEighty, Angle::Zero, Angle::Ninety),
    rotation_matrix(Angle::OneEighty, Angle::Zero, Angle::TwoSeventy),
    rotation_matrix(Angle::OneEighty, Angle::Ninety, Angle::Zero),
    rotation_matrix(Angle::OneEighty, Angle::Ninety, Angle::TwoSeventy),
    rotation_matrix(Angle::OneEighty, Angle::TwoSeventy, Angle::Zero),
    rotation_matrix(Angle::OneEighty, Angle::TwoSeventy, Angle::Ninety),
    rotation_matrix(Angle::TwoSeventy, Angle::Zero, Angle::Ninety),
    rotation_matrix(Angle::TwoSeventy, Angle::Zero, Angle::OneEighty),
    rotation_matrix(Angle::TwoSeventy, Angle::Ninety, Angle::Zero),
    rotation_matrix(Angle::TwoSeventy, Angle::Ninety, Angle::OneEighty),
    rotation_matrix(Angle::TwoSeventy, Angle::OneEighty, Angle::Zero),
    rotation_matrix(Angle::TwoSeventy, Angle::OneEighty, Angle::Ninety),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_count() {
        assert_eq!(ROTATIONS.len(), 24);
    }
}
