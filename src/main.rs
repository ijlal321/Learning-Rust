fn main(){
    let car = String::from("Red");

    let car1 = car;
    println!("Car color is {}", car1); // This will cause a compile-time error
}