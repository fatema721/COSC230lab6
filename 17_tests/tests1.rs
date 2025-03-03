// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
    let n = 10;
    println!("{} is even? {}", n, is_even(n));
    
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    #[test]
    fn is_even() {
        // TODO: Test the function `is_even` with some values.
        assert!(is_even(10));
        assert!(is_even(1));
    }
}