fn main() {
    let some_num = SomeEnum::<i32>::Type1(11); 
    some_num.check_var();

    let some_other_num = SomeEnum::Type1(11.12);
}

#[derive(Debug)]
enum SomeEnum <T> {
    Type1(T)
}

impl SomeEnum <i32>{

    fn check_var(&self) {
        println!("{:?}", self)
    }
}
