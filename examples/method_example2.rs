struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);

    println!("{}", rect1.width);

    println!("{}", rect1.width());
}
