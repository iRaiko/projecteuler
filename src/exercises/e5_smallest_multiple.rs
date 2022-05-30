pub fn smallest_multiple() -> usize {
    // 0-10 fit in 11-20 as divisors so dont need to be tested
    let divisors = vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11];
    // number has to be a multiple of 2520 or else it wont be divisble by all the numbers below 10
    for i in (2520..).step_by(2520) {
        if divisors.iter().all(|d| i % d == 0) {
            return i;
        }
    }
    return 0;
}

#[test]
fn test_multiples() {
    assert_eq!(smallest_multiple(), 232792560);
}