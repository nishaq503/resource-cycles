//! A `Resource` represents a node in the graph.

use crate::{Reflective, units::Units};

/// A `Resource` represents a node in the graph.
pub trait Resource: Reflective {
    /// The units of measurement for this resource.
    type Units: Units;

    /// The names of all known processes that consume this resource.
    fn known_consumers(&self) -> Vec<&str>;

    /// The names of all known processes that produce this resource.
    fn known_producers(&self) -> Vec<&str>;
}
