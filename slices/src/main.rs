fn main() {
    let mut cereals = [
        String::from("Cookie Crips"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];
    let first_two = &cereals[..2];
    println!("{first_two:?}");

    let mid_three = &cereals[1..5];
    println!("{mid_three:?}");

    let last_three = &mut cereals[2..];
    println!("{last_three:?}");
    last_three[2] = String::from("Lucky Charms");

    println!("{cereals:?}");

    let cookie_crips = &cereals[0];
    println!("{cookie_crips}");
    let cookie = &cereals[0][..6];
    println!("{cookie}");
    let cocoa_puffs = &cereals[4];
    println!("{cocoa_puffs}");

    let puffs = &cereals[4][7..];
    println!("{puffs}");
}
