//! Tests for the `Reflective` trait and its derive macro.

use resource_chains::Reflective;

/// `Foo`
pub struct Foo;

impl Reflective for Foo {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "Foo"
    }

    fn regex_pattern<'a>() -> &'a resource_chains::lazy_regex::Regex {
        resource_chains::lazy_regex::regex!(r"^(?i)foo$")
    }

    fn to_string(&self) -> String {
        "Foo".to_string()
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        match s {
            "foo" | "Foo" => Ok(Self),
            _ => Err(anyhow::anyhow!("Invalid input for Foo: {s}")),
        }
    }
}

/// `Foo2`
#[derive(Reflective)]
#[reflective(extra_names = ["FOO2"])]
pub struct Foo2;

/// `Bar`
pub struct Bar {
    /// `a`
    pub a: i32,
}

impl Reflective for Bar {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "Bar"
    }

    fn regex_pattern<'a>() -> &'a resource_chains::lazy_regex::Regex {
        resource_chains::lazy_regex::regex!(r"^Bar::a=(?P<a>-?\d+)$")
    }

    fn to_string(&self) -> String {
        format!("Bar::a={}", self.a)
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        if let Some(captures) = Self::regex_pattern().captures(s) {
            let a_str = captures
                .name("a")
                .ok_or_else(|| anyhow::anyhow!("Missing 'a' in input for Bar: {s}"))?
                .as_str();
            let a = a_str.parse::<i32>().map_err(|e| {
                anyhow::anyhow!("Invalid 'a' value in input for Bar: {s}. Error: {e}")
            })?;
            Ok(Self { a })
        } else {
            Err(anyhow::anyhow!("Invalid input for Bar: {s}"))
        }
    }
}

/// `FooBar`
#[derive(Reflective)]
pub struct FooBar {
    /// `a`
    pub a: i32,
    /// `b`
    pub b: f32,
}

/// `FooBar2`
#[derive(Reflective)]
pub struct FooBar2 {
    /// `foo`
    pub foo: Foo,
    /// `bar`
    pub bar: Bar,
}
