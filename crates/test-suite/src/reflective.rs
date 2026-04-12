//! Tests for the `Reflective` trait and its derive macro.

use resource_chains::Reflective;

/// `Foo`
pub struct Foo;

impl Reflective for Foo {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "Foo"
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        if s == "Foo" {
            Ok(Self)
        } else {
            Err(anyhow::anyhow!("Invalid input for Foo: {s}"))
        }
    }
}
