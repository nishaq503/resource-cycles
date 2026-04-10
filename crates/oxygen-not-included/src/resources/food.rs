//! Various food resources in the game "Oxygen Not Included".

use resource_cycles::{Reflective, Resource};

/// The [`OvagroNode`] produces [`OvagroFig`]s.
pub struct OvagroFig;

impl Reflective for OvagroFig {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "ovagro-fig"
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        match s {
            "ovagro-fig" | "OvagroFig" => Ok(Self),
            _ => Err(anyhow::anyhow!(
                "Invalid OvagroFig: {s}. Expected 'ovagro-fig' or 'OvagroFig'."
            )),
        }
    }
}

impl Resource for OvagroFig {
    type Units = crate::units::Kcal;

    fn known_consumers(&self) -> Vec<&str> {
        todo!()
    }

    fn known_producers(&self) -> Vec<&str> {
        vec!["ovagro-node"]
    }
}
