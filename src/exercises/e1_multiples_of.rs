use std::collections::HashSet;

pub fn sum_below(n: usize, multiples: &[usize]) -> usize {
    let mut set = HashSet::new();
    for i in multiples {
        multiples_of(n, *i, &mut set);
    }
    set.iter().sum()
}

fn multiples_of(max: usize, multiple: usize, set: &mut HashSet<usize>) {
    (0..max).step_by(multiple).for_each(|i| {
        set.insert(i);
    })
}


    #[test]
    fn test_multiples() {
        assert_eq!(sum_below(10, &[3, 5]), 23);
        assert_eq!(sum_below(6, &[3, 5]), 8);
    }
