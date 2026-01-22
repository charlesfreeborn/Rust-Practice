#[derive(Debug)]
struct Car {
    brand: String,
    color: String,
    year: String,
    is_new_energy: bool,
    price: f64,
}

// Standard Creation method
fn build_car(color: String, year: String, price: f64) -> Car {
    Car {
        brand: String::from("Tesla"),
        color: color,
        year: year,
        is_new_energy: true,
        price: price,
    }
}

// Simplified creation method
fn build_car2(color: String, year: String, price: f64) -> Car {
    Car {
        brand: String::from("Tesla"),
        // for function paratmers and structs with the same name, we can use abbreviation
        color,
        year,
        is_new_energy: true,
        price,
    }
}

fn main() {
    // Declare car type mut variable
    let mut car1 = build_car2(String::from("black"), String::from("2025-01-01"), 123.00);

    // Access and modify the structure using the dot notation
    car1.color = String::from("white");
    println!("car1 {:?}", car1);

    // Create a new structure instance based on the existing structure instance
    let mut car2 = Car {
        color: String::from("black"),
        ..car1
    };

    println!("car2 {:?}", car2);
}
