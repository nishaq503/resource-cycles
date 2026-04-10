//! The various kinds of water in the game. These include pure [water](https://oxygennotincluded.wiki.gg/wiki/Water), [polluted water](https://oxygennotincluded.wiki.gg/wiki/Polluted_Water), [salt water](https://oxygennotincluded.wiki.gg/wiki/Salt_Water), and [brine](https://oxygennotincluded.wiki.gg/wiki/Brine).

use resource_cycles::{Reflective, Resource, units::Kilogram};

/// Pure [Water](https://oxygennotincluded.wiki.gg/wiki/Water) is measured in kilograms (kg).
pub struct Water;

impl Reflective for Water {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "water"
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        match s {
            "water" | "Water" => Ok(Self),
            _ => Err(anyhow::anyhow!("Invalid resource: {s}. Expected 'water'.")),
        }
    }
}

impl Resource for Water {
    type Units = Kilogram;

    fn known_consumers(&self) -> Vec<&str> {
        vec!["ovagro-node"]
    }

    fn known_producers(&self) -> Vec<&str> {
        vec!["water-sieve"]
    }
}
