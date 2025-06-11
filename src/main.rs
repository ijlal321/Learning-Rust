#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64,
}

#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SupermarketItem>,
}

impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut operation: F)
    where
        F: FnMut(&mut SupermarketItem),
    {
        let mut market_len: usize = (self.items.len() as usize);
        while market_len > 0 {
            operation(&mut self.items[market_len - 1]);
            market_len -= 1;
        }
    }

    fn checkout<F>(self: Self, operation: F)
    where
        F: FnOnce(ShoppingCart),
    {
        operation(self);
    }
}

fn main() {
    let items = vec![
        SupermarketItem {
            name: "100".to_string(),
            price: 100.0,
        },
        SupermarketItem {
            name: "200".to_string(),
            price: 200.0,
        },
    ];
    let mut shopping_cart = ShoppingCart { items };

    shopping_cart.traverse_items(|item: &mut SupermarketItem| item.price *= 0.85);
    shopping_cart.traverse_items(|item: &mut SupermarketItem| item.name = item.name.to_lowercase());

    let mut total_price = 0.0;

    shopping_cart.checkout(|mut cart: ShoppingCart| {
        println!("{:?}", cart);

        cart.traverse_items(|item: &mut SupermarketItem| {
            total_price += item.price;
        });
    } );

    println!("{:.2} $", total_price)
}
