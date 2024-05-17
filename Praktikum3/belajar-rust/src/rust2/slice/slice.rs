// slice adalah cara untuk mengakses sebagian dari suatu nilai
// slice menggunakan simbol & sebelum nama variabel dan tanda kurung siku [..] untuk menentukan indeks awal dan akhir
fn slice() {
    // Contoh 1
    let s = String::from("hello slice");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("1. {} {}", hello, world);

    // Contoh 2
    let s = String::from("rust programming");
    let rust = &s[0..4];
    let programming = &s[5..16];
    println!("2. {} {}", rust, programming);

    // Contoh 3
    let s = String::from("ownership and borrowing");
    let ownership = &s[0..9];
    let borrowing = &s[14..23];
    println!("3. {} {}", ownership, borrowing);

    // Contoh 4
    let s = String::from("slicing in rust");
    let slicing = &s[0..7];
    let rust = &s[11..15];
    println!("4. {} {}", slicing, rust);

    // Contoh 5
    let s = String::from("mutable and immutable");
    let mutable = &s[0..7];
    let immutable = &s[12..21];
    println!("5. {} {}", mutable, immutable);

    // Contoh 6
    let s = String::from("safe and fast");
    let safe = &s[0..4];
    let fast = &s[9..13];
    println!("6. {} {}", safe, fast);

    // Contoh 7
    let s = String::from("rustaceans");
    let rust = &s[0..4];
    let aceans = &s[4..10];
    println!("7. {} {}", rust, aceans);

    // Contoh 8
    let s = String::from("hello slice");
    let hello = &s[0..5];
    let slice = &s[6..11];
    println!("8. {} {}", hello, slice);

    // Contoh 9
    let s = String::from("functional programming");
    let functional = &s[0..10];
    let programming = &s[11..22];
    println!("9. {} {}", functional, programming);

    // Contoh 10
    let s = String::from("learning rust");
    let learning = &s[0..8];
    let rust = &s[9..13];
    println!("10. {} {}", learning, rust);
}

fn main() {
    slice();
}