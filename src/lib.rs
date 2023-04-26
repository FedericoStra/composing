/*! Tools to compose functions.

This library exports two macros, [`compose_expr`] and [`compose_fn`],
which allow to easily compose expressions and functions respectively.
They both support right-to-left and left-to-right composition:

- right-to-left composition is achieved by separating the arguments with `,`;
- left-to-right composition is achieved by separating the arguments with `=>`.

# Examples

```
use composing::*;

fn plus_one(x: i32) -> i32 { x + 1 }
fn times_two(x: i32) -> i32 { x * 2 }
fn to_string(x: i32) -> String { x.to_string() }

let composition = compose_fn!(to_string, plus_one, times_two);
assert_eq!(composition(17), "35");

let composition = compose_fn!(times_two => plus_one => to_string);
assert_eq!(composition(17), "35");
```
*/

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/// Macro to compose expressions.
///
/// # Usage
///
/// The macro supports both right-to-left and left-to-right composition:
///
/// - right-to-left composition is achieved by giving the macro a comma-separated
/// list of expressions;
/// - left-to-right composition is achieved by giving the macro a list of expressions
/// separated by `=>`.
///
/// For instance,
/// - `compose_expr!(a, b, c, d)` expands to `a(b(c(d)))`;
/// - `compose_expr!(a => b => c => d)` expands to `d(c(b(a)))`.
///
/// # Examples
///
/// ## Right-to-left
///
/// ```
/// # use composing::compose_expr;
/// // equivalent to `|x| (x * 2) + 1`
/// let composition = |x| compose_expr!(|x| x + 1, |x| x * 2, x);
/// assert_eq!(composition(1), 3);
/// ```
///
/// ## Left-to-right
///
/// ```
/// # use composing::compose_expr;
/// // equivalent to `|x| (x + 1) * 2`
/// let composition = |x| compose_expr!(x => |x| x + 1 => |x| x * 2);
/// assert_eq!(composition(1), 4);
/// ```
#[macro_export]
macro_rules! compose_expr {
    () => {};
    ($expr:expr) => { $expr };

    // right-to-left composition
    ($head:expr, $($tail:expr),+) => { $head($crate::compose_expr!($($tail),+)) };

    // left-to-right composition
    ($first:expr => $second:expr) => { $second($first) };
    ($first:expr => $second:expr => $($tail:expr)=>+) => {
        $crate::compose_expr!($second($first) => $($tail)=>+)
    }
}

/// Macro to compose functions.
///
/// # Usage
///
/// The macro supports both right-to-left and left-to-right composition:
///
/// - right-to-left composition is achieved by giving the macro a comma-separated
/// list of functions;
/// - left-to-right composition is achieved by giving the macro a list of functions
/// separated by `=>`.
///
/// For instance,
/// - `compose_fn!(a, b, c, d)` expands to `|x| a(b(c(d(x))))`;
/// - `compose_fn!(a => b => c => d)` expands to `|x| d(c(b(a(x))))`.
///
/// # Examples
///
/// ## Right-to-left
///
/// ```
/// # use composing::compose_fn;
/// // equivalent to `|x| (x * 2) + 1`
/// let composition = compose_fn!(|x| x + 1, |x| x * 2);
/// assert_eq!(composition(1), 3);
/// ```
///
/// ## Left-to-right
///
/// ```
/// # use composing::compose_fn;
/// // equivalent to `|x| (x + 1) * 2`
/// let composition = compose_fn!(|x| x + 1 => |x| x * 2);
/// assert_eq!(composition(1), 4);
/// ```
#[macro_export]
macro_rules! compose_fn {
    () => { |x| x };
    ($($fns:expr),+) => { |x| $crate::compose_expr!($($fns),+, x) };
    ($($fns:expr)=>+) => { |x| $crate::compose_expr!(x => $($fns)=>+) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expr_right_to_left() {
        let composition = |x| compose_expr!(x);
        assert_eq!(composition(1), 1);

        let composition = |x| compose_expr!(|x| x + 1, x);
        assert_eq!(composition(1), 2);

        let composition = |x| compose_expr!(|x| x + 1, |x| x * 2, x);
        assert_eq!(composition(1), 3);

        let composition = |x| compose_expr!(|x| x * 2, |x| x + 1, x);
        assert_eq!(composition(1), 4);
    }

    #[test]
    fn expr_left_to_right() {
        let composition = |x| compose_expr!(x);
        assert_eq!(composition(1), 1);

        let composition = |x| compose_expr!(x => |x| x + 1);
        assert_eq!(composition(1), 2);

        let composition = |x| compose_expr!(x => |x| x * 2 => |x| x + 1);
        assert_eq!(composition(1), 3);

        let composition = |x| compose_expr!(x => |x| x + 1 => |x| x * 2);
        assert_eq!(composition(1), 4);
    }

    #[test]
    fn fn_right_to_left() {
        let composition = compose_fn!();
        assert_eq!(composition(1), 1);

        let composition = compose_fn!(|x| x + 1);
        assert_eq!(composition(1), 2);

        let composition = compose_fn!(|x| x + 1, |x| x * 2);
        assert_eq!(composition(1), 3);

        let composition = compose_fn!(|x| x * 2, |x| x + 1);
        assert_eq!(composition(1), 4);
    }

    #[test]
    fn fn_left_to_right() {
        let composition = compose_fn!();
        assert_eq!(composition(1), 1);

        let composition = compose_fn!(|x| x + 1);
        assert_eq!(composition(1), 2);

        let composition = compose_fn!( |x| x * 2 => |x| x + 1);
        assert_eq!(composition(1), 3);

        let composition = compose_fn!( |x| x + 1 => |x| x * 2);
        assert_eq!(composition(1), 4);
    }

    #[test]
    #[cfg(feature = "std")]
    fn advanced() {
        fn plus_one(x: i32) -> i32 {
            x + 1
        }
        fn times_two(x: i32) -> i32 {
            x * 2
        }
        fn to_string(x: i32) -> String {
            x.to_string()
        }

        let composition = compose_fn!(to_string, plus_one, times_two);
        assert_eq!(composition(17), "35");

        let composition = compose_fn!(times_two => plus_one => to_string);
        assert_eq!(composition(17), "35");
    }
}
