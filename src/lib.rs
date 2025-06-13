pub fn fib(n: i32) -> i32 {
    if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[test]
fn test_fib() {
    assert_eq!(fib(20), 6765);
}