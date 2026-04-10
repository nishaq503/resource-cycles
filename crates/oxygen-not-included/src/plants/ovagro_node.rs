//! An [Ovagro Node](https://oxygennotincluded.wiki.gg/wiki/Ovagro_Node).

use resource_cycles::{Process, Reflective};

/// An [`OvagroNode`] is a plant in the game.
pub struct OvagroNode;

impl Reflective for OvagroNode {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "ovagro-node"
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        match s {
            "ovagro-node" | "OvagroNode" => Ok(Self),
            _ => Err(anyhow::anyhow!(
                "Invalid OvagroNode: {s}. Expected 'ovagro-node' or 'OvagroNode'."
            )),
        }
    }
}

impl Process for OvagroNode {
    type TimePerIteration = crate::units::Cycle;

    fn consumed_resources(&self) -> Vec<(&str, f64)> {
        vec![("water", 90.0)]
    }

    fn produced_resources(&self) -> Vec<(&str, f64)> {
        todo!()
    }
}
