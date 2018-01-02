///! [Problem 4](https://projecteuler.net/problem=4)
///!
///!
///! A palindromic number reads the same both ways. The largest palindrome made
///! from the product of two 2-digit numbers is 9009 = 91 x 99.
///!
///! Find the largest palindrome made from the product of two 3-digit numbers.

// the problem space of this task is way to small to even attempt SIMD and
// parallelization. previous experience with these suggest, that CPU bound
// tasks need a much higher bandwidth to be worth it (~ 1_000_000_000_000++
// iterations).

pub fn largest_palindrome_from_three_digit_factors() -> u32 {
    let largest_factor = 999u32;
    let smallest_factor = 100u32;

    let mut largest_palindrome = 0;

    // iterate through all products and find largest palindrome
    // reverse the directions of the loops so they
    // start with large values. That means large product will be found
    // early.
    for i in (smallest_factor..largest_factor + 1).rev() {
        // since we know, that we are computing the product, it would be
        // pointless to iterate twice over equivalent combinations (e.g. 1 x 2
        // == 2 x 1)
        // this is easy to visualize by examining a small multiplication table.
        // a nested for loop traverses this structure from left to right, line
        // by line. Notice how the only unique values are in the diagonal from
        // cell 0,0 to 10,10. left and right from that diagonal value repeat.
        // this is because multiplication is commutative (2x3 == 3x2).
        //
        // 0 	1 	2 	3 	4 	5 	6 	7 	8 	9 	10
        // 1 	1 	2 	3 	4 	5 	6 	7 	8 	9 	10
        // 2 	2 	4 	6 	8 	10 	12 	14 	16 	18 	20
        // 3 	3 	6 	9 	12 	15 	18 	21 	24 	27 	30
        // 4 	4 	8 	12 	16 	20 	24 	28 	32 	36 	40
        // 5 	5 	10 	15 	20 	25 	30 	35 	40 	45 	50
        // 6 	6 	12 	18 	24 	30 	36 	42 	48 	54 	60
        // 7 	7 	14 	21 	28 	35 	42 	49 	56 	63 	70
        // 8 	8 	16 	24 	32 	40 	48 	56 	64 	72 	80
        // 9 	9 	18 	27 	36 	45 	54 	63 	72 	81 	90
        // 10 	10 	20 	30 	40 	50 	60 	70 	80 	90 	100
        //
        // we adjust the inner loop such that only one side of the diagonal
        // will be iterated.
        for j in (smallest_factor..i + 1).rev() {
            let product = i * j;

            // if the current product has gotten smaller than the largest
            // palindrome, searching any further makes no sense.
            // this assumes the iteration is REVERSED!
            if product < largest_palindrome {
                break;
            }
            // we inline a max function in here to avoid buffering all the
            // products in a Vec.
            if is_palindrome(product) {
                largest_palindrome = product;
            }
        }
    }

    // (oliver): I am unsatisfied with the amount of iteration done in this code.
    // another idea is to somehow iterate the numbers such that resulting
    // products are sorted (reverse), so that the first palindrome found must
    //be the largest. but again, I have no idea how to craft that iteration.

    if largest_palindrome == 0 {
        panic!("No palindrome found! This means there is a bug.");
    } else {
        largest_palindrome
    }
}

/// Checks if this number is a palindrome, by comparing the number to its
/// reverse.
/// If the comparison yields `true`, the number must be a palindrome.
fn is_palindrome(number: u32) -> bool {
    let mut remaining_digits = number;

    // Another strategy for detecting palindromes could be
    // to simply convert the number to a string and then reverse compare. I would expect that to have similar space-time behavior as this solution because in both cases there is buffering involved.
    // I wonder if there is some cute streaming solution?

    let mut digits = Vec::new();

    // we keep chopping off the last position of the number until nothing is
    // left
    // this works because we are assuming a decimal number system. if this
    // changes, take the base 10 and expose it as a parameter (radix)
    while remaining_digits != 0 {
        digits.push(remaining_digits % 10);
        remaining_digits = remaining_digits / 10;
    }

    // return early to avoid the clone.
    if digits.is_empty() {
        return false;
    }

    let mut reverse_digits = digits.clone();
    reverse_digits.reverse();

    digits == reverse_digits
}

#[cfg(test)]
mod tests {

    use problem_4::*;

    #[test]
    fn check_simple_cases() {
        assert_eq!(is_palindrome(7777), true);
        assert_eq!(is_palindrome(8888), true);
        assert_eq!(is_palindrome(1001), true);
        assert_eq!(is_palindrome(1221), true);
        assert_eq!(is_palindrome(12521), true);
        assert_eq!(is_palindrome(72627), true);
        assert_eq!(is_palindrome(1235), false);
        assert_eq!(is_palindrome(234325), false);
        assert_eq!(is_palindrome(932), false);
    }

    #[test]
    fn rust_ranges_inclusion() {
        let mut included = false;
        // ranges are exclusive for the second operand.
        // does this mean, this loop is a no-op?
        for _ in 1..1 {
            included = true;
        }

        //loop will never enter because range is essentially 1 -- 0.
        assert!(!included);
    }
}
