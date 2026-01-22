// use generic here where types are T
struct Point1<T> {
    x: T,
    y: T,
}

// 2. use generic but member can have different type
struct Point2<T, U> {
    x: T,
    y: U,
}

// 3. use generic in enum and the Option enum returns a value of some type T or no value None
enum Option<T> {
    Some(T),
    None,
}

// usinb generic in the method, we implemented the method get_x for the structure Point1<T>, used to retin the calue of the x member

impl<T> Point1<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    // 1. use generics in structures
    let int_point = Point1 { x: 5, y: 10 };
    let float_point = Point1 { x: 1.0, y: 4.0 };

    // 2. use generics in structures
    let p = Point2 { x: 1, y: 1.1 };

    // use generics in enum
    let option1 = Option::Some(1_i32);
    let option2 = Option::Some(1.00_f64);

    // use generic in metrics
    let x = int_point.get_x();
}
