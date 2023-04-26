use composing::compose_fn;

// Check that the macro can be called from another crate.

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
