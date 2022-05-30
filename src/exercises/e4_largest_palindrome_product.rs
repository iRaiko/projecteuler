
pub fn largest_palindrome(digits: usize) -> usize {
    let max = 10_usize.pow(digits as u32) - 1;
    let top_end = max - (10_usize.pow((digits - 1) as u32) - 1);
    let mid_end = max - ((10_usize.pow((digits - 1) as u32) - 1) / 2);
    let mut largest = 0;
    for i in (top_end..=max).rev().step_by(2) {
        for y in (mid_end..=max).rev().step_by(2) {
            let num = i * y;
            if is_palindrome(num) {
                if largest < num {
                    largest = num;
                }
            }
        }
    }
    largest
}


fn is_palindrome(mut n: usize) -> bool {
    let original = n;
    let mut reverse_n = 0;
    while n > 0 {
        reverse_n = reverse_n * 10 + n % 10;
        n = n / 10;
    }
    original == reverse_n
}

#[test]
fn test_largest_palindrome() 
{
    assert_eq!(largest_palindrome(2), 9009);
    assert_eq!(largest_palindrome(3), 906609);
}

#[test]
fn test_is_palindrome() 
{
    assert_eq!(is_palindrome(131), true);
    assert_eq!(is_palindrome(9009), true);
    assert_eq!(is_palindrome(2020202), true);
    assert_eq!(is_palindrome(1234), false);
}