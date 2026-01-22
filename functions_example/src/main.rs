fn another_function(first_name: &str, age: i32) {
    println!("My daughter's name is {first_name}, and she is {age}.");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello from the main function");

    another_function("Zoe Pearl", 17);

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is now: {x}");
}
