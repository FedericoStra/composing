# composing

> Tools to compose functions

[![crates.io](https://img.shields.io/crates/v/composing?logo=rust)](https://crates.io/crates/composing)
[![docs.rs](https://img.shields.io/docsrs/composing?logo=docsdotrs)](https://docs.rs/composing)
[![GitHub](https://img.shields.io/static/v1?label=github&message=FedericoStra/composing&color=brightgreen&logo=github)](https://github.com/FedericoStra/composing)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/FedericoStra/composing/rust.yml?logo=githubactions&logoColor=white)](https://github.com/FedericoStra/composing/actions/workflows/rust.yml)
[![Dependencies status](https://deps.rs/repo/github/FedericoStra/composing/status.svg)](https://deps.rs/repo/github/FedericoStra/composing)
[![MIT license](https://img.shields.io/crates/l/composing)](https://choosealicense.com/licenses/mit/)

This library exports two macros, [`compose_expr`] and [`compose_fn`],
which allow to easily compose expressions and functions respectively.

They both support right-to-left and left-to-right composition.

## Examples

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

[`compose_expr`]: https://docs.rs/composing/macro.compose_expr.html
[`compose_fn`]: https://docs.rs/composing/macro.compose_fn.html
