fn main() {
    // immutable default
    let ferris_id = 123456780;
    println!("ferris_id_card {}", ferris_id);

    // Mutable
    let mut ferris_address: &str = "Sunshine Beach No. 01";
    println!("Ferris lived in, {}", ferris_address);

    // reassign
    ferris_address = "Sunshine Beach No. 02";
    println!("Now, Ferris lives in, {}", ferris_address);
}
