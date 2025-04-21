#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

enum Subscription {
    Free,
    Basic(f64, u32), // price per month, num of months
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => println!("You have limited access to the site"),
            Subscription::Basic(price, months) => println!(
                "You have limited access to the site's premium features for {price} for {months} months"
            ),
            Subscription::Premium { tier } => println!(
                "You have full access to the site's premium features. Your tier is {tier:?}"
            ),
        }
    }
}

fn main() {
    let instance_1 = Subscription::Free;
    let instance_2 = Subscription::Basic(9.99, 12);
    let instance_3 = Subscription::Premium { tier: Tier::Gold };

    instance_1.summarize();
    instance_2.summarize();
    instance_3.summarize();
    
}
