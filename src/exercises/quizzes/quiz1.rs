// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
fn calculate_price_of_apples(quantity: i32) -> i32 {
    if quantity > 40 {
        quantity  // 每个苹果 1 rustbuck
    } else {
        quantity * 2  // 每个苹果 2 rustbucks
    }
}

fn main() {
    // You can optionally experiment here.
    let quantities = [35, 40, 41, 65];
    
    for &qty in &quantities {
        let price = calculate_price_of_apples(qty);
        println!("购买 {} 个苹果的价格: {} rustbucks", qty, price);
    }
    
    // 额外测试
    println!("\n额外测试:");
    for qty in 0..=50 {
        if qty % 5 == 0 {  // 每5个测试一次
            let price = calculate_price_of_apples(qty);
            println!("{} 个苹果: {} rustbucks", qty, price);
        }
    }
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}