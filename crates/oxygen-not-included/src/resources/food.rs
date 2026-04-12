//! Various food resources in the game "Oxygen Not Included".

use resource_chains::prelude::*;

/// The [`OvagroNode`](crate::plants::OvagroNode) produces [`OvagroFig`]s.
#[derive(Reflective)]
pub struct OvagroFig;

impl Resource for OvagroFig {
    type Units = crate::units::Kcal;

    fn known_consumers(&self) -> Vec<&str> {
        todo!()
    }

    fn known_producers(&self) -> Vec<&str> {
        vec!["ovagro-node"]
    }
}
