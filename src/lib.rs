#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

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
