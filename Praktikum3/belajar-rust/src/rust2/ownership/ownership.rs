// ownership adalah konsep kepemilikan pada Rust yang memungkinkan hanya satu variabel yang memiliki kepemilikan pada suatu waktu
// Ketika kepemilikan dialihkan, maka variabel yang sebelumnya memiliki kepemilikan tidak lagi valid
// Hal ini memungkinkan Rust untuk mencegah memory leak dan double free
fn ownership1() {
    let x = String::from("ownership"); // Membuat string baru "Halo" dan x adalah pemiliknya
    let y = x; // Mengalihkan kepemilikan string dari x ke y, sehingga x tidak lagi valid
    // println!("{}", x); // ERROR! x tidak lagi valid karena kepemilikannya telah dialihkan ke y
    println!("1. Hallo: {}", y); // y masih valid karena sekarang y yang memiliki kepemilikan
}

fn ownership2() {
    let a = 10;
    let b = a;
    println!("2. a = {}, b = {}", a, b);
}

fn ownership3() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec1;
    println!("3. {:?}", vec2);
}

fn ownership4() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("4. {}, world!", s2);
}

fn ownership5() {
    let num1 = 42;
    let num2 = num1;
    println!("5. num1 = {}, num2 = {}", num1, num2);
}

fn ownership6() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("6. {}, world!", s2);
}

fn ownership7() {
    let s = String::from("hello");
    let len = cal(&s);
    println!("7. The length of '{}' is {}.", s, len);
}

fn ownership8() {
    let mut s = String::from("hello");
    let len = change(&mut s);
    println!("8. The length of '{}' is {}.", s, len);
}

fn ownership9() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("9. {}", s);
}

fn ownership10() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("10. {}, world!", s2);
}


fn cal(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn main() {
    ownership1();
    ownership2();
    ownership3();
    ownership4();
    ownership5();
    ownership6();
    ownership7();
    ownership8();
    ownership9();
    ownership10();
}
