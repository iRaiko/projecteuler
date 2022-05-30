pub fn sum_square_difference(n: usize) -> usize {
    let sample: usize = (1..=n).map(|i| i * i).sum();
    let mut sum: usize = (1..=n).sum();
    sum = sum * sum;
    sum - sample
}

#[test]
fn test_sum_square_difference()
{
    assert_eq!(sum_square_difference(10), 2640);
    assert_eq!(sum_square_difference(100), 25164150);
}