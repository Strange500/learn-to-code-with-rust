const TOUCHDOWN_POINTS: i32 = 6;

type Season = str;
type Point = i32;
fn main() {
    let season: &Season = "Winter";
    let mut points_scored: Point = 28;
    points_scored = 35;

    let event_time = "06:00";
    let event_time = 6;

    println!(
        "In the {0} season, the team scored {points_scored} points (including {1} touchdown points). The event starts at l{2}.",
        season, TOUCHDOWN_POINTS, event_time
    );
    #[allow(unused_variables)]
    let beverage = "coke";
}
