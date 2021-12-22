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
#[derive(Clone, Copy, Debug)]
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

/// Produces the rotation matrix used for a rotation around the x axis in a counterclockwise
/// direction by the given angle.
const fn rotation_x(angle: Angle) -> [[i64; 3]; 3] {
    let sin_angle = sin(angle);
    let cos_angle = cos(angle);
    [
        [1, 0, 0],
        [0, cos_angle, -sin_angle],
        [0, sin_angle, cos_angle],
    ]
}

/// Produces the rotation matrix used for a rotation around the y axis in a counterclockwise
/// direction by the given angle.
const fn rotation_y(angle: Angle) -> [[i64; 3]; 3] {
    let sin_angle = sin(angle);
    let cos_angle = cos(angle);
    [
        [cos_angle, 0, sin_angle],
        [0, 1, 0],
        [-sin_angle, 0, cos_angle],
    ]
}

/// Produces the rotation matrix used for a rotation around the z axis in a counterclockwise
/// direction by the given angle.
const fn rotation_z(angle: Angle) -> [[i64; 3]; 3] {
    let sin_angle = sin(angle);
    let cos_angle = cos(angle);
    [
        [cos_angle, -sin_angle, 0],
        [sin_angle, cos_angle, 0],
        [0, 0, 1],
    ]
}

/// Calculates the dot product of the row of one matrix and column of a second.
const fn dot(mat_1: &[[i64; 3]; 3], mat_2: &[[i64; 3]; 3], idx_1: usize, idx_2: usize) -> i64 {
    mat_1[idx_1][0] * mat_2[0][idx_2]
        + mat_1[idx_1][1] * mat_2[1][idx_2]
        + mat_1[idx_1][2] * mat_2[2][idx_2]
}

/// Multiplies two rotation matrices together to produce a new one. We'll hardcode the
/// multiplication since we want this as a const function (can't use Matrix implementation of
/// multiply here).
const fn mult_mat(mat_1: [[i64; 3]; 3], mat_2: [[i64; 3]; 3]) -> [[i64; 3]; 3] {
    [
        [
            dot(&mat_1, &mat_2, 0, 0),
            dot(&mat_1, &mat_2, 0, 1),
            dot(&mat_1, &mat_2, 0, 2),
        ],
        [
            dot(&mat_1, &mat_2, 1, 0),
            dot(&mat_1, &mat_2, 1, 1),
            dot(&mat_1, &mat_2, 1, 2),
        ],
        [
            dot(&mat_1, &mat_2, 2, 0),
            dot(&mat_1, &mat_2, 2, 1),
            dot(&mat_1, &mat_2, 2, 2),
        ],
    ]
}

/// Matrix multiplication isn't commutative so this defines the order of rotation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum RotationOrder {
    XYZ,
    XZY,
    YXZ,
    YZX,
    ZXY,
    ZYX,
}

/// Produces the matrix for a given sequence of rotations with a given
/// angle set. Order of rotation is Z, Y, then X.
const fn rotation_matrix(x: Angle, y: Angle, z: Angle) -> Matrix<i64, 3, 3> {
    Matrix::new(mult_mat(
        mult_mat(rotation_x(x), rotation_y(y)),
        rotation_z(z),
    ))
}

// all the possible rotation matrices with all possible orders of multiplication.
// identity rotation is placed first.
pub const ROTATIONS: [Matrix<i64, 3, 3>; 24] = [
    rotation_matrix(Angle::Zero, Angle::Zero, Angle::Zero),
    rotation_matrix(Angle::Zero, Angle::Zero, Angle::Ninety),
    rotation_matrix(Angle::Ninety, Angle::OneEighty, Angle::OneEighty),
    rotation_matrix(Angle::Ninety, Angle::OneEighty, Angle::Zero),
    rotation_matrix(Angle::Zero, Angle::Zero, Angle::TwoSeventy),
    rotation_matrix(Angle::Zero, Angle::Ninety, Angle::TwoSeventy),
    rotation_matrix(Angle::Zero, Angle::TwoSeventy, Angle::Zero),
    rotation_matrix(Angle::Zero, Angle::TwoSeventy, Angle::OneEighty),
    rotation_matrix(Angle::Zero, Angle::Zero, Angle::OneEighty),
    rotation_matrix(Angle::Zero, Angle::OneEighty, Angle::TwoSeventy),
    rotation_matrix(Angle::Ninety, Angle::OneEighty, Angle::TwoSeventy),
    rotation_matrix(Angle::Zero, Angle::OneEighty, Angle::Ninety),
    rotation_matrix(Angle::Zero, Angle::OneEighty, Angle::OneEighty),
    rotation_matrix(Angle::Zero, Angle::Ninety, Angle::OneEighty),
    rotation_matrix(Angle::Ninety, Angle::Zero, Angle::TwoSeventy),
    rotation_matrix(Angle::Zero, Angle::Ninety, Angle::Ninety),
    rotation_matrix(Angle::Zero, Angle::Ninety, Angle::Zero),
    rotation_matrix(Angle::Ninety, Angle::Zero, Angle::Ninety),
    rotation_matrix(Angle::Ninety, Angle::Zero, Angle::OneEighty),
    rotation_matrix(Angle::Zero, Angle::TwoSeventy, Angle::Ninety),
    rotation_matrix(Angle::Ninety, Angle::OneEighty, Angle::Ninety),
    rotation_matrix(Angle::Zero, Angle::OneEighty, Angle::Zero),
    rotation_matrix(Angle::Ninety, Angle::Zero, Angle::Zero),
    rotation_matrix(Angle::Zero, Angle::TwoSeventy, Angle::TwoSeventy),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_count() {
        assert_eq!(ROTATIONS.into_iter().collect::<HashSet<_>>().len(), 24);
    }
}
