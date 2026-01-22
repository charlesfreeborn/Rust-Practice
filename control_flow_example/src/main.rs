fn main() {
    let condition = true;

    // 1. if branch assignment, the if statement block is an expression and the return value of the if expression can be assigned as a value to the variable
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // 2. Loop skipping and interruption
    for item in 1..=5 {
        if item == 2 {
            //  skip loop and go to next loop
            continue;
        }

        if item == 4 {
            break;
            // brekas the entire loop
        }
        println!("this Item is : {}", item);
    }

    // 3. Ownership transfer occurs in for loop
    let vec1: Vec<i32> = vec![1, 2, 3, 4, 5];
    for item in vec1.into_iter() {
        println!("Item: {}", item);
    }

    // 4. for loop borrows collection elements: immutable borrowing
    let vec2: Vec<i32> = vec![1, 2, 3, 4, 5];
    for item in &vec2 {
        println!("Item: {}!", item);
    }

    // here vec2 still has ownership
    println!("{:?}", vec2);

    // 5. while vs for loop
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    for element in a.iter() {
        println!("the velue is: {} !", element);
    }

    // 6. loop loop as expression
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}
