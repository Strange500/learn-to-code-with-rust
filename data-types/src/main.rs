fn main() {
    let int = 1_337;

    let casted = int as i16;

    let float = 31.14564589;

    println!("{:.3}", float);

    let with_milk = true;

    let with_sugar = false;

    let is_my_type_of_coffe = with_sugar && with_sugar;

    let array = [1, 2, 4, 5];

    dbg!(array);

    let tuple = (5, 5.0, true, array);

    dbg!(tuple);
}
