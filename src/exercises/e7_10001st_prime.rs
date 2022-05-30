use super::prime_iter::Primes;

fn nth_prime(n: usize) -> usize
{
    let mut primes = Primes::new();
    let mut iter = primes.iter();
    for _ in 0..=n
    {
        iter.next();
    }
    primes[n - 1]
}


#[test]
fn test_nth_prime()
{
    assert_eq!(nth_prime(6), 13);
    assert_eq!(nth_prime(10001), 104743);
}