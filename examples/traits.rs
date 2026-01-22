#[derive(Debug)]
// trait is the keyword

struct WildGoose {
    color: String,
}

impl WildGoose {
    fn new() -> Self {
        WildGoose {
            color: "gray".to_string(),
        }
    }

    fn inhabit(&self) {
        println!("Wild geese perch by the lake");
    }
}

// Swallow structure
struct Swallow {
    color: String,
}

// swallow own method
impl Swallow {
    fn new() -> Self {
        Swallow {
            color: "black".to_string(),
        }
    }

    fn build_nest(&self) {
        println!("Swallows build nests under the eaves")
    }
}

// trait
trait MigrantBird {
    fn migrant(&self) -> String;
}

impl MigrantBird for WildGoose {
    fn migrant(&self) -> String {
        "Geese fly in a V-shaped formation".to_string()
    }
}

impl MigrantBird for Swallow {
    fn migrant(&self) -> String {
        "Swallow fly fast, but have to rest frequently".to_string()
    }
}

fn main() {
    println!(
        "calling the {:?}",
        WildGoose {
            color: "White".to_string()
        }
    );
}
