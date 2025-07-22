// pub fn fib(n: i32) -> i32 {
//     let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
//     let psi = (1.0 - 5.0_f32.sqrt()) / 2.0;
//     ((phi.powi(n) - psi.powi(n)) / 5.0_f32.sqrt()) as i32
// }

pub fn fib(n: i32) -> i32 {
    if n <= 2 { 1 } else { fib(n - 1) + fib(n - 2) }
}

// writing a comment to get the actions to trigger

#[test]
fn test_fib() {
    assert_eq!(fib(20), 6765);
}