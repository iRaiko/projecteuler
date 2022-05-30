fn fibonacci_sum_even(max_n: usize) -> usize {
    let mut sum = 0;
    let mut first_term = 1;
    let mut second_term = 2;
    while second_term < max_n {
        if second_term % 2 == 0 {
            sum += second_term;
        }
        let temp = second_term;
        second_term += first_term;
        first_term = temp;
    }
    sum
}

#[test]
fn test_fibonacci_sum_even() {
    assert_eq!(fibonacci_sum_even(10), 10);
    assert_eq!(fibonacci_sum_even(50), 44);
}