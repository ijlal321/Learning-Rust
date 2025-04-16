fn main(){
    let season = "summer";

    let ans = match season {
        "spring" => {1;},
        "summer" => println!("It's summer!"),
        _ => println!("It's not spring or summer!"),
    };
}