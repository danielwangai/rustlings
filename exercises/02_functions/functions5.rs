// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        // example 1
        let number1: i32 = 10;
        let number1_square = square(number1);
        let expected_square1 = 100;
        assert_eq!(expected_square1, number1_square);
        // example 2
        let number2: i32 = 10;
        let number2_square = square(number2);
        let expected_square2 = 100;
        assert_eq!(expected_square2, number2_square);
    }
}
