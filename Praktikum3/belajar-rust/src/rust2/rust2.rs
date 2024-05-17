// ownership adalah konsep kepemilikan pada Rust yang memungkinkan hanya satu variabel yang memiliki kepemilikan pada suatu waktu
// Ketika kepemilikan dialihkan, maka variabel yang sebelumnya memiliki kepemilikan tidak lagi valid
// Hal ini memungkinkan Rust untuk mencegah memory leak dan double free
fn ownership() {
    let x = String::from("Halo"); // Membuat string baru "Halo" dan x adalah pemiliknya
    let y = x; // Mengalihkan kepemilikan string dari x ke y, sehingga x tidak lagi valid
    // println!("{}", x); // ERROR! x tidak lagi valid karena kepemilikannya telah dialihkan ke y
    println!("ownership: {}", y); // y masih valid karena sekarang y yang memiliki kepemilikan
}

// reference adalah cara untuk mengakses nilai tanpa mengambil kepemilikan
// reference menggunakan simbol & sebelum nama variabel
// reference tidak memungkinkan perubahan nilai dari variabel yang di-reference
fn reference() {
    let x = String::from("Halo");
    let y = cal(&x); // y adalah reference dari x
    println!("reference: {}", x); // x masih valid
    println!("reference: {}", y); // y masih valid
}

fn cal(s:&String) -> usize {
    s.len() // mengembalikan panjang string
}

// borrowing adalah cara untuk meminjam kepemilikan dari variabel lain tanpa mengambil kepemilikan
// borrowing menggunakan simbol &mut sebelum nama variabel
// borrowing memungkinkan perubahan nilai dari variabel yang di-borrow
fn borrowing() {
    let mut x = String::from("Halo");
    let y = change(&mut x); // y adalah peminjam kepemilikan dari x
    println!("borrowing: {}", x); // x masih valid
    println!("borrowing: {}", y); // y masih valid
}

fn change(s:&mut String) -> usize {
    s.push_str(", Dunia!"); // menambahkan ", Dunia!" ke string
    s.len() // mengembalikan panjang string
}

// clone adalah cara untuk membuat salinan dari suatu nilai yang memiliki kepemilikan
// clone memungkinkan kita untuk memiliki dua variabel yang memiliki kepemilikan pada suatu waktu
fn clone() {
    let x = String::from("Halo");
    let y = x.clone(); // y adalah salinan dari x
    println!("clone: {}", x); // x masih valid
    println!("clone: {}", y); // y masih valid
}

// copy adalah cara untuk membuat salinan dari suatu nilai yang memiliki kepemilikan
// copy hanya berlaku untuk tipe data primitif seperti integer, float, boolean, dan char
// copy memungkinkan kita untuk memiliki dua variabel yang memiliki kepemilikan pada suatu waktu
fn copy() {
    let x = 5;
    let y = x; // y adalah salinan dari x
    println!("copy: {}", x); // x masih valid
    println!("copy: {}", y); // y masih valid
}

// scope adalah waktu dimana suatu variabel masih valid dan dapat diakses
// Rust menggunakan aturan bahwa suatu variabel hanya valid di dalam blok dimana variabel tersebut dideklarasikan
fn scope() {
    let x = 5; // x dideklarasikan di dalam blok ini
    println!("scope: {}", x); // x masih valid
    { // blok baru dimulai
        let y = 10; // y dideklarasikan di dalam blok ini
        println!("scope: {}", y); // y masih valid
        println!("scope: {}", y+x);
    } // blok baru berakhir
    // println!("{}", y); // ERROR! y tidak lagi valid karena blok baru telah berakhir
}

// mutable adalah cara untuk membuat variabel yang dapat diubah nilainya
// mutable menggunakan kata kunci mut sebelum nama variabel
fn mutable() {
    let mut x = 5; // x adalah variabel mutable
    println!("before mutable: {}", x); // x masih valid
    x = 10; // mengubah nilai x
    println!("after mutable: {}", x); // x masih valid
}

// dangling adalah keadaan dimana suatu variabel masih valid meskipun variabel yang seharusnya memiliki kepemilikan telah dihapus
// Rust mencegah dangling dengan menggunakan aturan bahwa suatu variabel hanya valid jika memiliki kepemilikan
fn dangling() {
    let s = dangling_reference(); // s adalah reference ke String s
    println!("dangling: {}", s); // s masih valid
    // println!("dangling: {}", s.len()); // ERROR! s tidak lagi valid karena String s telah dihapus
}

fn dangling_reference() -> String {
    let s = String::from("hello");
    s // Mengembalikan referensi ke String s
}

// slice adalah cara untuk mengakses sebagian dari suatu nilai
// slice menggunakan simbol & sebelum nama variabel dan tanda kurung siku [..] untuk menentukan indeks awal dan akhir
fn slice() {
    let s = String::from("hello");
    let slice = &s[0..2]; // slice adalah sebagian dari s yang dimulai dari indeks 0 dan berakhir sebelum indeks 2
    println!("slice: {} {}", slice, s); // slice masih valid
}

fn main() {
    ownership();
    reference();
    borrowing();
    clone();
    copy();
    scope();
    mutable();
    dangling();
    slice();
}