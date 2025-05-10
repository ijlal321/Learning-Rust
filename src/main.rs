fn some_fun() -> &'static str {
    let some_string = String::from("123");
    &some_string
}

fn main() {

    let train_a = {
        let name = String::from("123");
        Train{name:&name}

    };



}

