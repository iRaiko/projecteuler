pub fn largest_prime_factor(n: usize) -> usize {
    let factors = find_prime_factors(n);
    *factors.iter().max().unwrap()
}

fn find_prime_factors(mut n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let mut primes = crate::exercises::prime_iter::Primes::new();
    while n > 1 {
        let iter = primes.iter();
        for i in iter {
            if n % i == 0 {
                factors.push(i);
                n = n / i;
                break;
            }
        }
    }
    factors
}

#[test]
fn test_largest_prime_factor() {
    assert_eq!(largest_prime_factor(13195), 29);
    assert_eq!(largest_prime_factor(600851475143), 6857);
}