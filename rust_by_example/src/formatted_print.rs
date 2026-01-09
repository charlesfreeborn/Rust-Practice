fn main() {
    // Generally, `{}` automatically replaces any arguments.

    println!("{} days!", 31);

    // positional arguments can be used. you can specify an integer inside the {}
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // different formatting can be invoked by specifying the format character after a `:`
    println!("Base 10:              {}", 69420);
    println!("Base 2 (binary):      {:b}", 69420);
    println!("Base 8 (octal):       {:o}", 69420);
    println!("Base 16 (hexadecimal)  {:x}", 69420);

    // you can right-justify text with a specified width.
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes.

    println!("{number:0>5}", number = 1);

    // left-adjust by flipping the sign. that is use the less than < symbol
    println!("{number:0<5}", number = 1);

    // you can use named arguments in the format specifier by appedning a `$`

    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make the correct number of arguments are used
    println!("My name is {0}. {1} {0}", "Bond", "James");

    // struct Structure(i32);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
