use super::Sensor;
use crate::common::math::Matrix;

/// Represents a sensor link between two sensors that overlap.
/// Contains a reference to the source sensor (that you're transforming), and the destination sensor (that you're transforming onto)
/// and the rotation matrix used to perform the transformation. After applying rotation, there'll be a translation also
/// that needs to be applied to align the dots.
#[derive(Debug)]
pub struct SensorLink<'a> {
    pub source: &'a Sensor<'a>,
    pub destination: &'a Sensor<'a>,
    pub rotation: &'a Matrix<i64, 3, 3>,
    pub translation: (i64, i64, i64),
}
