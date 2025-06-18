pub fn fib(n: i32) -> i32 {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let psi = (1.0 - 5.0_f32.sqrt()) / 2.0;
    ((phi.powi(n) - psi.powi(n)) / 5.0_f32.sqrt()) as i32
}

#[test]
fn test_fib() {
    assert_eq!(fib(20), 6765);
}
