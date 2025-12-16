// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// Don't worry about the function bodies themselves, we are only interested in
// the signatures for now.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Fix the function signature.
fn sale_price(price: i64) -> i64 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sale_price() {
        let even_price = 100;
        let even_sale_price = sale_price(even_price);
        let even_discount = even_price - even_sale_price;
        assert_eq!(even_sale_price, 90);
        assert_eq!(even_discount, 10);

        // odd price
        let odd_price = 101;
        let odd_sale_price = sale_price(odd_price);
        let odd_discount = odd_price - odd_sale_price;
        assert_eq!(odd_sale_price, 98);
        assert_eq!(odd_discount, 3);
    }
}
