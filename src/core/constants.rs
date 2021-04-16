// General constants
/// Circle constant representing the ratio  between circumference and radius. Equal to two times
/// PI.
pub const TAU: f64 = std::f64::consts::TAU;

// Time related constants
// These constants are defined in seconds, in the Earth time referential.
/// Duration of a minute in seconds.
pub const MINUTE: f64 = 60.0;
/// Duration of an hour in seconds.
pub const HOUR: f64 = MINUTE * 60.0;
/// Duration of an Earth day in seconds.
pub const DAY: f64 = HOUR * 24.0;
/// Year alias for 365.25 days.
pub const YEAR: f64 = DAY * 365.25;

// Distance related constants
// These constants are defined in meters.
/// [Astronaumical unit](https://en.wikipedia.org/wiki/Astronomical_unit).
pub const ASTRONAUMICAL_UNIT: f64 = 1.495978707e11;

// Conversion constants
/// Conversion from degrees to radians.
pub const DEG2RAD: f64 = TAU / 360.0;
/// Conversion from radians to degrees.
pub const RAD2DEG: f64 = 360.0 / TAU;
