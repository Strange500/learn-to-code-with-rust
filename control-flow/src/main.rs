fn main() {
    println!("{}", factorial(5));
    println!("{}", factorial(5));
}

fn color_to_number(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial(n: i32) -> i32 {
    let mut current = n;
    let mut result = 1;

    while current > 0 {
        result *= current;
        current -= 1;
    }
    result
}

fn rec_factorial(n: i32) -> i32 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
