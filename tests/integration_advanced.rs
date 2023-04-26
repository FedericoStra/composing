use composing::compose_fn;

#[test]
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
    fn parse(s: String) -> i32 {
        s.parse().unwrap_or_default()
    }

    let composition = compose_fn!(parse, to_string, plus_one, times_two);
    assert_eq!(composition(17), 35);

    let composition = compose_fn!(times_two => plus_one => to_string => parse);
    assert_eq!(composition(17), 35);
}
