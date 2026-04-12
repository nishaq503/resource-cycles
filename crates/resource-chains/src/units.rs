//! Units of measurement for various resources.

use crate::Reflective;

/// `Units` of measurement for a resource, such as "kg", "m", "s", etc.
pub trait Units: Reflective {}

/// `Time` represents a unit of time, such as "s", "hr", "day", etc.
pub trait Time: Units {
    /// Returns the number of seconds in one unit of this time.
    fn seconds_per_unit() -> f64;
}
