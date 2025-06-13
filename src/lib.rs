pub fn fib(n: i32) -> i32 {
    if n <= 2 {
        1
    } else {
        let mut a = 1;
        let mut b = 1;
        for _ in 3..n + 1 {
            let tmp = b;
            b += a;
            a = tmp;
        }
        b
    }
}
