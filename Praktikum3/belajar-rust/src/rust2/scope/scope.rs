// scope adalah waktu dimana suatu variabel masih valid dan dapat diakses
// Rust menggunakan aturan bahwa suatu variabel hanya valid di dalam blok dimana variabel tersebut dideklarasikan
fn scope() {
    // Contoh 1
    {
        let s = String::from("scope");
        println!("1. {}", s);
    }
    // println!("{}", s); // Error: s tidak lagi valid di luar scope

    // Contoh 2
    {
        let x = 10;
        println!("2. {}", x);
    }
    // println!("{}", x); // Error: x tidak lagi valid di luar scope

    // Contoh 3
    {
        let vec = vec![1, 2, 3];
        println!("3. {:?}", vec);
    }
    // println!("{:?}", vec); // Error: vec tidak lagi valid di luar scope

    // Contoh 4
    {
        let s = String::from("rust");
        println!("4. {}", s);
    }
    // println!("{}", s); // Error: s tidak lagi valid di luar scope

    // Contoh 5
    {
        let n = 42;
        println!("5. {}", n);
    }
    // println!("{}", n); // Error: n tidak lagi valid di luar scope

    // Contoh 6
    {
        let s = String::from("scope");
        println!("6. {}", s);
    }
    // println!("{}", s); // Error: s tidak lagi valid di luar scope

    // Contoh 7
    {
        let num = 100;
        println!("7. {}", num);
    }
    // println!("{}", num); // Error: num tidak lagi valid di luar scope

    // Contoh 8
    {
        let vec = vec![4, 5, 6];
        println!("8. {:?}", vec);
    }
    // println!("{:?}", vec); // Error: vec tidak lagi valid di luar scope

    // Contoh 9
    {
        let s = String::from("example");
        println!("9. {}", s);
    }
    // println!("{}", s); // Error: s tidak lagi valid di luar scope

    // Contoh 10
    {
        let x = 7;
        println!("10. {}", x);
    }
    // println!("{}", x); // Error: x tidak lagi valid di luar scope
}

fn main() {
    scope();
}