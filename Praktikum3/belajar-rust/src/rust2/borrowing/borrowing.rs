// borrowing adalah cara untuk meminjam kepemilikan dari variabel lain tanpa mengambil kepemilikan
// borrowing menggunakan simbol &mut sebelum nama variabel
// borrowing memungkinkan perubahan nilai dari variabel yang di-borrow
fn borrowing() {
    // Contoh 1
    let mut s1 = String::from("borrowing");
    change(&mut s1);
    println!("1. {}", s1);

    // Contoh 2
    let mut s2 = String::from("world");
    append_exclamation(&mut s2);
    println!("2. {}", s2);

    // Contoh 3
    let mut vec1 = vec![1, 2, 3];
    add_element(&mut vec1, 4);
    println!("3. {:?}", vec1);

    // Contoh 4
    let mut num1 = 42;
    increment(&mut num1);
    println!("4. {}", num1);

    // Contoh 5
    let mut s3 = String::from("rust");
    to_uppercase(&mut s3);
    println!("5. {}", s3);

    // Contoh 6
    let mut num2 = 10;
    double_value(&mut num2);
    println!("6. {}", num2);

    // Contoh 7
    let mut vec2 = vec![4, 5, 6];
    remove_last(&mut vec2);
    println!("7. {:?}", vec2);

    // Contoh 8
    let mut s4 = String::from("borrowing");
    reverse(&mut s4);
    println!("8. {}", s4);

    // Contoh 9
    let mut vec3 = vec![7, 8, 9];
    clear_vector(&mut vec3);
    println!("9. {:?}", vec3);

    // Contoh 10
    let mut num3 = 56;
    halve_value(&mut num3);
    println!("10. {}", num3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn add_element(v: &mut Vec<i32>, elem: i32) {
    v.push(elem);
}

fn increment(n: &mut i32) {
    *n += 1;
}

fn to_uppercase(s: &mut String) {
    *s = s.to_uppercase();
}

fn double_value(n: &mut i32) {
    *n *= 2;
}

fn remove_last(v: &mut Vec<i32>) {
    v.pop();
}

fn reverse(s: &mut String) {
    let reversed: String = s.chars().rev().collect();
    *s = reversed;
}

fn clear_vector(v: &mut Vec<i32>) {
    v.clear();
}

fn halve_value(n: &mut i32) {
    *n /= 2;
}

fn main() {
    borrowing();
}