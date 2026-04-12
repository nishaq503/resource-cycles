//! Studying resource creation and consumption pathways using graphs.

pub mod prelude;
pub mod process;
pub mod resource;
pub mod units;

extern crate resource_chains_derive;

pub use resource_chains_derive::Reflective;

/// A `Reflective` type is one that can be named as a `String` and parsed from a `String`.
///
/// # Example
///
/// ```rust
/// use resource_chains::Reflective;
///
/// struct Foo;
///
/// impl Reflective for Foo {
///     type ParseError = String;
///
///     fn type_name() -> &'static str {
///        "foo"
///     }
///
///     fn parse(s: &str) -> Result<Self, Self::ParseError> {
///         if s == "foo" {
///             Ok(Self)
///         } else {
///             Err(format!("Invalid input: {s}. Expected 'foo'."))
///         }
///     }
/// }
///
/// assert_eq!(Foo::type_name(), "foo");
/// let foo = Foo::parse("foo").unwrap();
/// ```
pub trait Reflective: Sized {
    /// The type of error that can occur when parsing an instance of the type from a string.
    type ParseError;

    /// The name of the type.
    fn type_name() -> &'static str;

    /// Parse an instance of the type from a string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed into an instance of the type.
    fn parse(s: &str) -> Result<Self, Self::ParseError>;
}

impl Reflective for () {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "()"
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        match s {
            "()" => Ok(()),
            _ => Err(anyhow::anyhow!("Invalid unit: {s}. Expected '()'.")),
        }
    }
}

/// A macro to implement `Reflective` for primitive types.
macro_rules! impl_reflective_for_primitive {
    ($($t:ty),*) => {
        $(
            impl Reflective for $t {
                type ParseError = anyhow::Error;

                fn type_name() -> &'static str {
                    stringify!($t)
                }

                fn parse(s: &str) -> Result<Self, Self::ParseError> {
                    s.parse::<$t>().map_err(|e| {
                        anyhow::anyhow!("Failed to parse '{}' as {}: {}", s, Self::type_name(), e)
                    })
                }
            }
        )*
    };
}

impl_reflective_for_primitive!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool, char, String
);
