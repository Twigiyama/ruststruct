#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    } 

    fn new(name: &str) -> Shuttle{
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}

struct Colour (u8,u8,u8); //This represents R,G,B colour values
struct Point (u8, u8, u8); //This represents X,Y,Z points

fn get_y(p: Point) -> u8 {
    p.1
}

struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.height * self.width
    }

    fn scale(&mut self, scale: f64) {
        self.height *= scale;
        self.width *= scale;
    }

    fn new(w: f64, h: f64) -> Rectangle {
        Rectangle { width: w, height: h }
    }
}

fn main() {
    println!("Hello, world!");

    let mut vehicle = Shuttle::new("Finnegan");

    let mut vehicle2: Shuttle = Shuttle {
        ..vehicle.clone()
    };

    println!("This shuttle is {}", vehicle.name);

    println!("The vehicle is {:?}", vehicle);
    println!("The vehicle is {:?}", vehicle2);

    let vehicle_name = vehicle.get_name();
    println!("Using the method in Shuttle the name is {vehicle_name}");

    println!("The starting value for propellany is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("The new value for propellant is {}", vehicle.propellant);

    let red = Colour(255,0,0);
    println!("First value is {}", red.0);

    let coord = Point(34,7,8);
    let y = get_y(coord);
    println! ("The y is {y}");

    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}
