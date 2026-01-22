use std::collections::HashMap;
fn main() {
    // Vector, type is tuple (user, balance)
    let user_list: Vec<(&str, i32)> = vec![
        ("Charles", 10000),
        ("Martha", 1000),
        ("Favour", 100),
        ("Zoe", 10),
    ];

    // use iterator and collect metho to convert array to HashMap
    let mut user_map: HashMap<&str, i32> = user_list.into_iter().collect();
    println!("{:?}", user_map);

    // Get corresponding value via hashmap[key]
    let charles_balance = user_map["Charles"];
    println!("{:?}", charles_balance);

    // return value as none, if key doesn't exist
    let trent_balance: Option<&i32> = user_map.get("Trent");
    println!("{:?}", trent_balance);

    // override existing value
    let old = user_map.insert("Charles", 20000);
    assert_eq!(old, Some(10000));

    let v = user_map.entry("Trent").or_insert(1);
    assert_eq!(*v, 1);

    let v = user_map.entry("Trent").or_insert(2);
    assert_eq!(*v, 1);
}
