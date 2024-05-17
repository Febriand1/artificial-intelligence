// dangling adalah keadaan dimana suatu variabel masih valid meskipun variabel yang seharusnya memiliki kepemilikan telah dihapus
// Rust mencegah dangling dengan menggunakan aturan bahwa suatu variabel hanya valid jika memiliki kepemilikan
fn dangling() {
    // Contoh 1
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    // Contoh 2
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    // Contoh 3
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    // Contoh 4
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    // Contoh 5
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    // Contoh 6
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    // Contoh 7
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    // Contoh 8
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    // Contoh 9
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    // Contoh 10
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

// fn dangle() -> &String { // Error: mengembalikan reference ke data yang sudah di-drop
fn dangle() -> String { // Deklarasi
    let s = String::from("dangling");
    s // Mengembalikan string, bukan reference
}

fn main() {
    dangling();
}