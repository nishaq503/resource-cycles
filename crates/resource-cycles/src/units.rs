//! Units of measurement for various resources.

use crate::Reflective;

/// `Units` of measurement for a resource, such as "kg", "m", "s", etc.
pub trait Units: Reflective {}

/// `Time` represents a unit of time, such as "s", "hr", "day", etc.
pub trait Time: Units {
    /// Returns the number of seconds in one unit of this time.
    fn seconds_per_unit() -> f64;
}

/// A `Second` is a unit of time equal to one second.
pub struct Second;

impl Reflective for Second {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "s"
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        match s {
            "s" | "second" | "Second" | "sec" => Ok(Self),
            _ => Err(anyhow::anyhow!(
                "Invalid time unit: {s}. Expected 's', 'second', 'Second', or 'sec'."
            )),
        }
    }
}

impl Units for Second {}

impl Time for Second {
    fn seconds_per_unit() -> f64 {
        1.0
    }
}

/// A `Kilogram` is a unit of mass equal to one kilogram.
pub struct Kilogram;

impl Reflective for Kilogram {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "kg"
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        match s {
            "kilogram" | "kg" => Ok(Self),
            _ => Err(anyhow::anyhow!(
                "Invalid mass unit: {s}. Expected 'kilogram' or 'kg'."
            )),
        }
    }
}

impl Units for Kilogram {}
