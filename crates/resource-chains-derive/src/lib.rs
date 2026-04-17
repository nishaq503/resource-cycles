//! Derive macros for the `resource-cycles` crate.

#![expect(clippy::missing_panics_doc, clippy::unwrap_used)]

use proc_macro::TokenStream;

mod reflective;

/// Derive the `Reflective` trait.
///
/// By default, the type name (with hyphens instead of camel case) will be used as the string representation of the type. For example, `MyStruct` will be
/// represented as `"my-struct"`. The struct can be parsed from this string, as well as the actual struct name, i.e. `"MyStruct"`. You can specify any
/// additional string representations using the `reflective` attribute, e.g. `#[reflective(extra_names = ["ms", "Ms"])]`.
///
/// # Example
///
/// ```rust
/// use resource_chains::Reflective;
///
/// #[derive(Reflective)]
/// struct Foo;
///
/// #[derive(Reflective)]
/// #[reflective(extra_names = ["b"])]
/// struct Bar;
///
/// #[derive(Reflective)]
/// #[reflective(extra_names = ["fb", "FB"])]
/// struct FooBar;
/// ```
///
/// In this example, the `Foo` struct can be parsed from the string `"foo"` or `"Foo"`, the `Bar` struct can be parsed from the string `"bar"` or `"b"`, and the
/// `FooBar` struct can be parsed from the string `"foo-bar"`, `"fb"`, or `"FB"`.
#[proc_macro_derive(Reflective, attributes(reflective))]
pub fn reflective_derive(item: TokenStream) -> TokenStream {
    reflective::derive(item.into()).unwrap().into()
}
