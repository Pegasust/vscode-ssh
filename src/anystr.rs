//! AnyStr abstract
//!
//! This allows us accept a wide variety of string by wrapping upon
//! the trait [AsRef<str>] into [AnyStr] trait along with some utilities

/// AnyStr allows us to pass in &[str], [String], &[String]
/// and can convert to [String] using [any_str_to_string] func
pub trait AnyStr: AsRef<str> {}
impl<T> AnyStr for T where T: AsRef<str> {}

/// Turns AnyStr into [String]
/// 
/// # Examples
/// 
/// ```
/// use vscode_ssh::anystr::*;
/// fn ok(s: String) {}
/// let literal = "hello world";
/// let owned = "hello world".to_string();
/// let mut mut_literal = "hello world";
///
/// ok(any_str_to_string(literal));
/// ok(any_str_to_string(mut_literal));
/// {
///     let borrowed = &owned;
///     ok(any_str_to_string(borrowed))
/// };
/// {
///     let mut mut_borrowed = &owned;
///     ok(any_str_to_string(mut_borrowed));
/// }
/// ok(any_str_to_string(owned));
/// ```
pub fn any_str_to_string<_AnyStr: AnyStr>(s: _AnyStr) -> String {
    s.as_ref().to_string()
}

/// A shortcut to create a None that is compatible with [Option<_AnyStr:AnyStr>]
/// 
/// # Examples
/// 
/// ```compile_fail
/// use vscode_ssh::anystr::*;
/// fn option_anystr<S: AnyStr>(opt: Option<S>) {}
/// option_anystr(None); // compile failure: no matching type for S: AnyStr
/// ```
///
/// The compilation problem is solved by providing concrete type for AnyStr
/// ```
/// use vscode_ssh::anystr::*;
/// fn option_anystr<S: AnyStr>(opt: Option<S>) {}
/// option_anystr(anystr_none())
/// ```
pub const fn anystr_none() -> Option<String> {
    None
}

/// A symmetrical shortcut to [anystr_none()]
///
///
/// # Examples
///
/// ```
/// use vscode_ssh::anystr::{anystr_some, AnyStr};
/// use std::borrow::{Borrow, BorrowMut};
/// fn ok<S: AnyStr>(opt: Option<S>) {
///     matches!(opt, Option::Some(_));
/// }
/// ok(anystr_some("hello world"));
/// ok(anystr_some("hello_world".to_string()));
/// ```
pub fn anystr_some<_AnyStr: AnyStr>(s: _AnyStr) -> Option<_AnyStr> {
    Some(s)
}
