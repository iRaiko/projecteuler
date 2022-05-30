pub struct Primes {
    primes: Vec<usize>,
}

impl Primes {
    pub fn new() -> Primes {
        Primes { primes: vec![2, 3] }
    }
    pub fn iter(&mut self) -> PrimeIter<'_> {
        PrimeIter {
            primes: &mut self.primes,
            index: 0,
        }
    }
}
pub struct PrimeIter<'a> {
    primes: &'a mut Vec<usize>,
    index: usize,
}
impl<'a> Iterator for PrimeIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.primes.get(self.index) {
            Some(prime) => {
                self.index += 1;
                Some(*prime)
            }
            None => {
                for next in (self.primes[self.index - 1] + 2..).step_by(2) {
                    if self
                        .primes
                        .iter()
                        .take_while(|&&i| i * i <= next)
                        .all(|&p| next % p != 0)
                    {
                        self.index += 1;
                        self.primes.push(next);
                        return Some(next);
                    }
                }
                None
            }
        }
    }
}