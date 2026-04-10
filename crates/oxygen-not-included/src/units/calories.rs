//! Calories measure food in the game.

use resource_cycles::{Reflective, units::Units};

/// In "Oxygen Not Included", a "kcal" (kilocalorie) is the unit of energy provided by food.
pub struct Kcal;

impl Reflective for Kcal {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "kcal"
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        match s {
            "kcal" | "Kcal" => Ok(Self),
            _ => Err(anyhow::anyhow!(
                "Invalid Kcal: {s}. Expected 'kcal' or 'Kcal'."
            )),
        }
    }
}

impl Units for Kcal {}
