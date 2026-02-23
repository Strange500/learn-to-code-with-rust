#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Self::Free => println!("You have limietd access to the site"),
            Self::Basic(price, months) => println!(
                "You have limmited access to the site's premium features for {price} for {months} months",
            ),
            Self::Premium { tier } => println!("Your tier is {tier:?}"),
        }
    }
}
fn main() {
    let free = Subscription::Free;
    let Basic = Subscription::Basic(13.2, 24);
    let premium = Subscription::Premium {
        tier: Tier::Platinum,
    };

    free.summarize();
    Basic.summarize();
    premium.summarize();
}
