/// Circle constant representing the ratio  between circumference and radius. Equal to two times
/// PI.
pub const TAU: f64 = std::f64::consts::TAU;

/// Conversion from degrees to radians.
pub const DEG2RAD: f64 = TAU / 360.0;

/// Conversion from radians to degrees.
pub const RAD2DEG: f64 = 360.0 / TAU;

/// Duration of an Earth day in seconds.
pub const DAY: f64 = 86400.0;

/// Duration of an hour in seconds.
pub const HOUR: f64 = 3600.0;

/// Duration of a minute in seconds.
pub const MINUTE: f64 = 60.0;
