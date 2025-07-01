//! # dyn_path
//!
//! dyn_path is a set of macros that permit you access objects
//! that have `.get()` methods that return `Option<T>` in a nested
//! way.
//!
//! It is as specific as it looks, but most libraries that parse
//! data interchange languages have a "Value" that contains other
//! "Value"s inside. And casually all the "Value"s have a `.get()`
//! method, a generic `.get()` method in fact.
//!
//! How does this work? Just like JavaScript.
//! ```rust
//! use serde_json::json;
//! use dyn_path::dyn_access;
//!
//! let object = json!({
//!     "very": {
//!         "nested": {
//!             "value": [
//!                 "hello",
//!                 "world"
//!             ]
//!         }
//!     }
//! });
//!
//! let hello = dyn_access!(object.very.nested.value[0]).unwrap();
//! let world = dyn_access!(object.very.nested.value[1]).unwrap();
//!
//! assert_eq!(hello, "hello");
//! assert_eq!(world, "world");
//! ```
//! This is also useful for nested `HashMap`s but the difference is
//! that you will actually get a compile time error if you are wrong
//! with the type.
//! ```rust
//! use std::collections::HashMap;
//! use dyn_path::dyn_access;
//!
//! let map: HashMap<String, HashMap<String, HashMap<i32, ()>>> = HashMap::new();
//!
//! dyn_access!(map.nested.value[&0]); // since we don't have any real value this will return None.
//! ```
//! Check the available macro documentation to learn more about how to use
//! the specific macros.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
pub extern crate alloc;

#[cfg(test)]
mod test;

/// # dyn_access
/// The `dyn_access` has a specific use-case, which is
/// accessing very deeply nested values in parsed structures.
///
/// For example, imagine you have an API, which you only need some
/// data for. Usually in JavaScript you simply fetch a value and
/// access it dinamically with a path like `value?.nested?.nested[0]`
/// then simply check for undefined.
///
/// This macro permits you access to very nested objects with javascript
/// like indexers, with the exception you don't need to use `?`, as you
/// get an `Option<T>` instead.
///
/// This macro is recursive and will stop working when the value
/// doesn't have a `.get` method that returns an Option<T>.
///
/// To invoke this macro you just use a path like
/// ```rust
/// use serde_json::json;
/// use dyn_path::dyn_access;
///
/// let object = json!({
///     "very": {
///         "nested": {
///             "value": [
///                 "hello",
///                 "world"
///             ]
///         }
///     }
/// });
///
/// let hello = dyn_access!(object.very.nested.value[0]).unwrap();
/// let world = dyn_access!(object.very.nested.value[1]).unwrap();
///
/// assert_eq!(hello, "hello");
/// assert_eq!(world, "world");
/// ```
/// You also have indices available to you, whether it is
/// for an array or an object.
///
/// Notice how the first element is the name of the variable,
/// you can have an expression in there with parenthesis like
/// `(value.parse::<serde_json::Value>()?).very.nested.value`,
/// the parenthesis are due to parsing system limitation since
/// this is a `macro_rules` and not a `proc_macro`.
#[macro_export]
macro_rules! dyn_access {
    ($head:ident $($rest:tt)*) => {{
        $crate::dyn_access!(($head) $($rest)*)
    }};

    (($head:expr) $($rest:tt)*) => {{
        let __ = Some(&($head));
        $crate::dyn_access!(@recurse __, $($rest)*)
    }};

    (@recurse $acc:expr, . $field:ident $($rest:tt)*) => {{
        let __ = $acc.and_then(|v| v.get(::core::stringify!($field)));
        $crate::dyn_access!(@recurse __, $($rest)*)
    }};

    (@recurse $acc:expr, [$idx:expr] $($rest:tt)*) => {{
        let __ = $acc.and_then(|v| v.get($idx));
        $crate::dyn_access!(@recurse __, $($rest)*)
    }};

    (@recurse $acc:expr,) => {{ $acc }};
}

/// # dyn_path
/// The `dyn_path` macro just acts as a Display for the `dyn_access`
/// macro, meaning that this just generates a precomputed `String`
/// of the input path.
///
/// The syntax is essentially the same, with the only difference this
/// doesn't have a "head", meaning that you don't need to specify a source
/// from where to access something, the path is hypotetical.
///
/// An example invocation of this macro is
/// ```rust
/// use dyn_path::dyn_path;
///
/// let display_path = dyn_path!(nested.path.at[1 + 1].with["no"]["head"]);
///
/// assert_eq!(display_path, r#"nested.path.at[2].with["no"]["head"]"#);
/// ```
/// Notice how the macro pre-computes the indexes and generates the target string.
#[cfg(any(feature = "alloc", feature = "std"))]
#[macro_export]
macro_rules! dyn_path {
    ($head:ident $($rest:tt)*) => {{
        use ::core::fmt::Write;
        $crate::__import_alloc!()
        let mut __ = ::core::stringify!($head).to_string();
        $crate::dyn_path!(@recurse __, $($rest)*)
    }};

    (@recurse $acc:expr, . $field:ident $($rest:tt)*) => {{
        let _ = ::core::write!($acc, ".{}", ::core::stringify!($field));
        $crate::dyn_path!(@recurse $acc, $($rest)*)
    }};

    (@recurse $acc:expr, [$idx:expr] $($rest:tt)*) => {{
        let _ = ::core::write!($acc, "[{:?}]", ($idx));
        $crate::dyn_path!(@recurse $acc, $($rest)*)
    }};

    (@recurse $acc:expr,) => {{ $acc }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "alloc")]
macro_rules! __import_alloc { () => { use $crate::alloc::string::ToString; }; }

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "alloc"))]
macro_rules! __import_alloc { () => {}; }
