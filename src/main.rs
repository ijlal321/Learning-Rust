#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64, 
    passengers: u32,
}

impl Flight {

    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Flight {
            origin,
            destination,
            price,
            passengers,
        }
    }
    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination;
    }

    fn increase_price(&mut self) {
        self.price *= 1.20;
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
    
}

fn main() {
    let mut some_flight = Flight::new(
        String::from("New York"),
        String::from("Los Angeles"),
        300.0,
        150,    
    );

    some_flight.change_destination(String::from("San Francisco"));
    some_flight.increase_price();
    some_flight.itinerary();
    println!("Some FLight: {:?}", some_flight);


    let some_new_flight = Flight{
        origin: String::from("New York"),
        destination: String::from("Hello World"),
        ..some_flight
    };

    println!("Some New Flight: {:?}", some_new_flight);
    println!("Some Flight: {:?}", some_flight);
}

