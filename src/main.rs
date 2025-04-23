fn main() {
    let my_option = MyOption::Some("hello");
    let none_option = MyOption::<i32>::None;

    println!("{:?}  ,  {:?}", my_option, none_option);

    let my_option_res = my_option.unwrap("not working");
    let none_res = none_option.unwrap(11);

    println!("{}.", my_option_res);
    println!("{}.", none_res);
    
}

impl <T> MyOption<T> {
    fn unwrap(self, default_res: T) -> T {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => default_res
        }
    }
}


#[derive(Debug, Clone, Copy)]
enum MyOption<T> {
    Some(T),
    None
}