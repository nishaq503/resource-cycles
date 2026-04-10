//! Units used to measure time in the game "Oxygen Not Included".

use resource_cycles::{
    Reflective,
    units::{Time, Units},
};

/// In "Oxygen Not Included", a "[cycle](https://oxygennotincluded.wiki.gg/wiki/Cycles)" is 600 seconds.
pub struct Cycle;

impl Reflective for Cycle {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "cycle"
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        match s {
            "cycle" | "Cycle" => Ok(Self),
            _ => Err(anyhow::anyhow!(
                "Invalid Cycle: {s}. Expected 'cycle' or 'Cycle'."
            )),
        }
    }
}

impl Units for Cycle {}

impl Time for Cycle {
    fn seconds_per_unit() -> f64 {
        600.0
    }
}
