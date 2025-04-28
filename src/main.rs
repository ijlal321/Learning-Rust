fn main() {
    let resturant_a: Restaurant = Restaurant{
        reservations:11,
        has_mice_infestation: true
    };

    println!("{:?}", resturant_a.chef_special());
    println!("{:?}", resturant_a.deliver_burger("123 Elm Street"));

    let resturant_b: Restaurant = Restaurant{
        reservations:15,
        has_mice_infestation: false
    };

    println!("{:?}", resturant_b.chef_special());
    println!("{:?}", resturant_b.deliver_burger(""));
    println!("{:?}", resturant_b.deliver_burger("Some add"));

}

#[derive(Debug)]
struct Food {
    name: String,
}

struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}   

impl Restaurant {

    fn chef_special(&self) -> Option<Food> {
        
        if self.has_mice_infestation == true {
            None
        }
        else if self.reservations < 12 {
            Some(Food{name: String::from("Uni Sashimi")})
        }
        else{
            Some(Food{name: String::from("Strip Steak")})
        }

    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation == true {
            Err(String::from("Sorry, we have a mice problem"))
        }
        else if address.is_empty() {
            Err(String::from("No delivery address specified"))
        }
        else{
            Ok(Food{name: String::from("Burger")})
        }
    }

}

