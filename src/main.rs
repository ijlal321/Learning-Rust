#[allow(unused)]
use std::fmt::{Debug, Display, Formatter, Result};
use std::{fs::canonicalize, path::PrefixComponent};

trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) {
        println!("{}", self.get_data());
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

struct Coffee<T> {
    kind: T,
    Milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T) -> Self {
        Coffee {
            kind,
            Milk: Milk::Almond,
            ounces: 100,
        }
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }
    fn get_data(&self) -> String {
        format!("A delicious {} ounce of {}", self.ounces, self.kind)
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(
            formatter,
            "Your Coffee Sire:: {:?} {:?} {:?}",
            self.kind, self.Milk, self.ounces
        )
    }
}
#[derive(Debug)]

struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32,
}

impl Soda {
    fn new() -> Self {
        Soda {
            calories: 100,
            price: 100.25,
            flavor: "hello world".to_string(),
            percentage: 100,
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }
    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories: {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, " {} Soda ", self.flavor)
    }
}

impl Clone for Soda{
    fn clone(&self) -> Self {
        Soda { calories: self.calories, price: self.price, flavor: self.flavor.clone(), percentage: self.percentage }
    }
}

impl PartialEq for Soda{
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda{

}


fn main() {
    let mut latte = Coffee::new("hello world");
    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);

    let cappuccino = Coffee::new("Hello WOrld".to_string());
    println!("{}", cappuccino.get_data());


    let pepsi = Soda::new();
    println!("{}", pepsi);
    let mut coke = pepsi.clone();
    coke.consume();

    println!("{:?}", coke);

}
