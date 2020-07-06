pub mod raw_html;

pub use raw_html::RawHTML;

use crate::token;

pub fn if_auth_borrow<'a, T: ?Sized>(a: &'a T, b: &'a T) -> &'a T {
    if token::is_authenticated() {
        &a
    } else {
        &b
    }
}

pub fn if_auth<T>(a: T, b: T) -> T {
    if token::is_authenticated() {
        a
    } else {
        b
    }
}
