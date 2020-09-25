use cargo_snippet::snippet;

#[snippet]
fn adder(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test_say_hello() {
    assert_eq!(adder(0, 1), 1);
}
