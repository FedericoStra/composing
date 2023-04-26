use composing::compose_expr;

// Check that the macro can be called from another crate.

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
