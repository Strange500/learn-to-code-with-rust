fn main() {
    apply_to_job(35, "Rust dev");

    println!("{}", is_even(2));

    println!("{:#?}", test("az"));
}

fn apply_to_job(number: u32, title: &str) {
    println!("I'm am applying to {number} {title} jobs")
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn test(text: &str) -> (bool, bool) {
    (text.contains('a'), text.contains("z"))
}
