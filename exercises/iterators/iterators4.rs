// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // 1..=num is a range that includes the number num
    // fold is a function that takes an initial value and a closure with two arguments
    // the closure takes an accumulator and an element and returns the next accumulator
    // the closure is called for each element in the range
    // the final accumulator is returned
    // the closure multiplies the accumulator by the element

    // num = 4        acc   x
    // 1  = 1 * 1  <=  1  *  1
    // 2  = 1 * 2  <=  1  *  2
    // 6  = 2 * 3  <=  2  *  3
    // 24 = 6 * 4  <=  6  *  4
    (1..=num).fold(1, |acc, x| acc * x)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
