use crate::common::math::Matrix;

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

/// Const function that produces a rotation matrix given sine and cosine values for the angles
/// of each of the three axis. Rotation is applied in the order x, y, and then z.
const fn rotation_matrix(x: Angle, y: Angle, z: Angle) -> Matrix<i64, 3, 3> {
    let (sin_x, sin_y, sin_z, cos_x, cos_y, cos_z) =
        (sin(x), sin(y), sin(z), cos(x), cos(y), cos(z));
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

// Rotations to apply
const ROTATION_POS_Z_0_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::Zero, Angle::Zero);
const ROTATION_POS_Z_90_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::Zero, Angle::Ninety);
const ROTATION_POS_Z_180_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::Zero, Angle::OneEighty);
const ROTATION_POS_Z_270_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::Zero, Angle::TwoSeventy);

const ROTATION_NEG_Z_0_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::OneEighty, Angle::Zero);
const ROTATION_NEG_Z_90_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::OneEighty, Angle::Ninety);
const ROTATION_NEG_Z_180_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::OneEighty, Angle::OneEighty);
const ROTATION_NEG_Z_270_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::OneEighty, Angle::TwoSeventy);

const ROTATION_POS_X_0_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::Ninety, Angle::Zero);
const ROTATION_POS_X_90_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::Ninety, Angle::Ninety);
const ROTATION_POS_X_180_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::Ninety, Angle::OneEighty);
const ROTATION_POS_X_270_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::Ninety, Angle::TwoSeventy);

const ROTATION_NEG_X_0_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::TwoSeventy, Angle::Zero);
const ROTATION_NEG_X_90_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::TwoSeventy, Angle::Ninety);
const ROTATION_NEG_X_180_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::TwoSeventy, Angle::OneEighty);
const ROTATION_NEG_X_270_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Zero, Angle::TwoSeventy, Angle::TwoSeventy);

const ROTATION_POS_Y_0_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::TwoSeventy, Angle::Zero, Angle::Zero);
const ROTATION_POS_Y_90_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::TwoSeventy, Angle::Zero, Angle::Ninety);
const ROTATION_POS_Y_180_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::TwoSeventy, Angle::Zero, Angle::OneEighty);
const ROTATION_POS_Y_270_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::TwoSeventy, Angle::Zero, Angle::TwoSeventy);

const ROTATION_NEG_Y_0_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Ninety, Angle::Zero, Angle::Zero);
const ROTATION_NEG_Y_90_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Ninety, Angle::Zero, Angle::Ninety);
const ROTATION_NEG_Y_180_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Ninety, Angle::Zero, Angle::OneEighty);
const ROTATION_NEG_Y_270_DEG: Matrix<i64, 3, 3> =
    rotation_matrix(Angle::Ninety, Angle::Zero, Angle::TwoSeventy);

// all the valid orientations for the scanners.
const ROTATIONS: [&Matrix<i64, 3, 3>; 24] = [
    &ROTATION_POS_X_0_DEG,
    &ROTATION_POS_X_90_DEG,
    &ROTATION_POS_X_180_DEG,
    &ROTATION_POS_X_270_DEG,
    &ROTATION_NEG_X_0_DEG,
    &ROTATION_NEG_X_90_DEG,
    &ROTATION_NEG_X_180_DEG,
    &ROTATION_NEG_X_270_DEG,
    &ROTATION_POS_Y_0_DEG,
    &ROTATION_POS_Y_90_DEG,
    &ROTATION_POS_Y_180_DEG,
    &ROTATION_POS_Y_270_DEG,
    &ROTATION_NEG_Y_0_DEG,
    &ROTATION_NEG_Y_90_DEG,
    &ROTATION_NEG_Y_180_DEG,
    &ROTATION_NEG_Y_270_DEG,
    &ROTATION_POS_Z_0_DEG,
    &ROTATION_POS_Z_90_DEG,
    &ROTATION_POS_Z_180_DEG,
    &ROTATION_POS_Z_270_DEG,
    &ROTATION_NEG_Z_0_DEG,
    &ROTATION_NEG_Z_90_DEG,
    &ROTATION_NEG_Z_180_DEG,
    &ROTATION_NEG_Z_270_DEG,
];
