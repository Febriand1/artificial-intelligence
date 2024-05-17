// copy adalah cara untuk membuat salinan dari suatu nilai yang memiliki kepemilikan
// copy hanya berlaku untuk tipe data primitif seperti integer, float, boolean, dan char
// copy memungkinkan kita untuk memiliki dua variabel yang memiliki kepemilikan pada suatu waktu
fn copy() {
    // Contoh 1
    let x = "copy";
    let y = x;
    println!("1. x = {}, y = {}", x, y);

    // Contoh 2
    let a = true;
    let b = a;
    println!("2. a = {}, b = {}", a, b);

    // Contoh 3
    let c = 'c';
    let d = c;
    println!("3. c = {}, d = {}", c, d);

    // Contoh 4
    let f1 = 1.23;
    let f2 = f1;
    println!("4. f1 = {}, f2 = {}", f1, f2);

    // Contoh 5
    let n1 = 100;
    let n2 = n1;
    println!("5. n1 = {}, n2 = {}", n1, n2);

    // Contoh 6
    let t1 = (1, 2);
    let t2 = t1;
    println!("6. t1 = {:?}, t2 = {:?}", t1, t2);

    // Contoh 7
    let b1 = [1, 2, 3];
    let b2 = b1;
    println!("7. b1 = {:?}, b2 = {:?}", b1, b2);

    // Contoh 8
    let f3 = 4.56;
    let f4 = f3;
    println!("8. f3 = {}, f4 = {}", f3, f4);

    // Contoh 9
    let x1 = -42;
    let x2 = x1;
    println!("9. x1 = {}, x2 = {}", x1, x2);

    // Contoh 10
    let b3 = false;
    let b4 = b3;
    println!("10. b3 = {}, b4 = {}", b3, b4);
}

fn main() {
    copy();
}