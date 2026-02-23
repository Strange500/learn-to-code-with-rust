#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Flight {
        Flight {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    fn increase_price(&mut self) {
        self.price = self.price * 1.2;
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {
    let mut flight = Flight::new(String::from("Paris"), String::from("Tokyo"), 13.5, 50);
    flight.change_destination(String::from("Montdidier"));
    println!("{:#?}", flight);
    flight.increase_price();
    println!("{:#?}", flight);
    flight.itinerary();

    let second_flight = Flight {
        origin: flight.origin.clone(),
        destination: flight.origin.clone(),
        ..flight
    };

    second_flight.itinerary();
    flight.itinerary();
}
