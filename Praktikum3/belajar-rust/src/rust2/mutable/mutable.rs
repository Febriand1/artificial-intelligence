// mutable adalah cara untuk membuat variabel yang dapat diubah nilainya
// mutable menggunakan kata kunci mut sebelum nama variabel
fn mutable() {
    // Contoh 1
    let mut s = String::from("hello");
    s.push_str(", mutable");
    println!("1. {}", s);

    // Contoh 2
    let mut x = 5;
    x += 1;
    println!("2. {}", x);

    // Contoh 3
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!("3. {:?}", vec);

    // Contoh 4
    let mut s = String::from("rust");
    s.push_str(" programming");
    println!("4. {}", s);

    // Contoh 5
    let mut y = 10;
    y *= 2;
    println!("5. {}", y);

    // Contoh 6
    let mut vec = vec![4, 5, 6];
    vec.pop();
    println!("6. {:?}", vec);

    // Contoh 7
    let mut s = String::from("mutable");
    s.replace_range(0..4, "mut");
    println!("7. {}", s);

    // Contoh 8
    let mut z = 100;
    z /= 4;
    println!("8. {}", z);

    // Contoh 9
    let mut vec = vec![7, 8, 9];
    vec.clear();
    println!("9. {:?}", vec);

    // Contoh 10
    let mut s = String::from("example");
    s.push('!');
    println!("10. {}", s);
}

fn main() {
    mutable();
}