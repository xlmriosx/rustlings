fn factorial(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    let start = 1;
    let end = num;
    let fac_array: Vec<u64> = (start..=end).collect();
    fac_array.into_iter().product::<u64>()
}

fn main() {
    // You can optionally experiment here.
    // let start = 2;
    // let end = 4;
    // let fac_array: Vec<u64> = (start..=end).collect();
    // let result_array = fac_array.into_iter().fold(1, |ant, pres| ant * pres);
    // println!("{result_array:#?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
