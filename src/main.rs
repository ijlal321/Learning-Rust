use std::{collections::HashMap, vec};


fn main() {
    let mut sauces_to_meals: HashMap<&str, Vec<&str>> = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    let res = sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);
    
    if res.is_some() {
        println!("value was already present, so updated");
    }

    let res = sauces_to_meals.remove("Mayonnaise");
    match res {
        Some(ingredients) => println!("removed values are {:?}", ingredients),
        None => println!("Nothing is removed.")
    }

    let res = sauces_to_meals.get("Mustard");
    match res{
        Some(ingredients) => println!("These are ingredients: {:?}", ingredients),
        None=> println!("Nothung found")
    }

    sauces_to_meals.entry("Soy Sauce").or_insert(vec!["Sushi", "Dumplings"]);

    println!("final saouces to meals are: \n {:#?}", sauces_to_meals);

}