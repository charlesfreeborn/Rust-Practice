// enum BlockChain {
//     BitCoin,
//     Ethereum,
//     Starknet,
//     Solana,
// }

// fn main() {
//     let block_chain = BlockChain::Solana;
//     match block_chain {
//         BlockChain::BitCoin => println!("BitCoin"),
//         // X | Y - similar to logical OR
//         BlockChain::Ethereum | BlockChain::Starknet => {
//             println!("Ethereum or Starknet");
//         }
//         // Use .. to represent all posibilities
//         _ => println!("Solana"),
//     };
// }
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        // get bound value from the matching pattern such as radiux, width, size
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    }
}
struct Point {
    x: i32,
    y: i32,
}

fn process_point(point: Point) {
    match point {
        Point { x: 0, y: 0 } => println!("Coordinates are at the origin"),
        Point { x, y } => println!("Coordinates are at ({x}, {y})"),
    }
}

fn main() {
    let circle = Shape::Circle(3.0);
    let rectangle = Shape::Rectangle(4.0, 5.0);
    let square = Shape::Square(2.0);

    // 1. call function to output area of each shape
    println!("The area of a circle is: {}", calculate_area(&circle));
    println!(
        "The area of the rectangle is: {}",
        calculate_area(&rectangle)
    );
    println!("The area of the square: {}", calculate_area(&square));

    //    2. match the pattern using matching for assignment
    let area = match circle {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    };
    println!("The area of the circle is: {area}");

    // 3. Deconstruct the structure
    let point1 = Point { x: 0, y: 0 };
    let point2 = Point { x: 3, y: 7 };
    process_point(point1);
    process_point(point2);

    // 4. if let simple matching
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }
}
