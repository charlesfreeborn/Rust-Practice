fn main() {
    let mut v1 = vec![1, 2, 3, 4, 5];

    // Access third element using the specified position
    let third: &i32 = &v1[2];
    println!("the third element is {}", third);

    // access using the .get()
    match v1.get(2) {
        Some(third) => println!("The third element is {third}"),
        None => println!("The specified element does not exist"),
    }

    for i in &mut v1 {
        *i += 10 // look out for this line
    }
    println!("v1 = {:?}", v1);

    let mut v2: Vec<i32> = vec![1, 2];
    assert!(!v2.is_empty());
    v2.insert(2, 3);
    assert_eq!(v2.remove(1), 2);
    assert_eq!(v2.pop(), Some(3));
    v2.clear();
}
