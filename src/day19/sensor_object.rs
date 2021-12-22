/// Allows us to distinguish positions of sensors
/// and beacons.
#[derive(Eq, Hash, PartialEq)]
pub enum SensorObject {
    Sensor((i64, i64, i64)),
    Beacon((i64, i64, i64)),
}
