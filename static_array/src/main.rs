fn main() {
    // array = [type: length]
    let a = [3u8; 5];
    println!("Static type of basic type {:?}", a);

    let b: [String; 3] = std::array::from_fn(|_i| String::from("rust"));
    println!("Data of non-basic type: {:?}", b);

    let c = [9, 8, 7, 6, 5];
    // Direct access via subscript
    let first = c[0];
    println!("The first item in {:?} is {}", c, first);
    let second = c[1];

    // arrays in two dimensional array
    let arrays: [[u8; 5]; 2] = [a, c];
    println!("Arrays with two-dimensional arrays: {:?}", arrays);
}
