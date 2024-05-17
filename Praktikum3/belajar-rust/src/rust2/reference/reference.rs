// reference adalah cara untuk mengakses nilai tanpa mengambil kepemilikan
// reference menggunakan simbol & sebelum nama variabel
// reference tidak memungkinkan perubahan nilai dari variabel yang di-reference
fn reference() {
    // contoh 1
    let x = String::from("Reference");
    let y = cal(&x); // y adalah reference dari x
    println!("1. reference: {}", x); // x masih valid
    println!("1. reference: {}", y); // y masih valid

    // contoh 2
    let s2 = String::from("world");
    let first_char = get_first_char(&s2);
    println!("2. First character of '{}' is {}", s2, first_char);

    // contoh 3
    let vec1 = vec![1, 2, 3];
    let vec2 = print_vector(&vec1);
    println!("3. {:?}", vec2);

    // contoh 4
    let mut vec1 = vec![1, 2, 3];
    let vec2 = change_vector(&mut vec1);
    let vec3 = print_vector(&vec1);
    println!("4. {:?},", vec2);
    println!("4. {:?},", vec3);
    

    // contoh 5
    let number = 42;
    let number = print_number(&number);
    println!("5. {}", number);

    // contoh 6
    let mut number = 42;
    let number = change_number(&mut number);
    println!("6. {}", number);

    // contoh 7
    let s5 = String::from("borrow");
    let borrow_length = calculate_length(&s5);
    println!("7. Length of '{}' is {}", s5, borrow_length);

    // contoh 8
    let num1 = 100;
    let is_positive = check_positive(&num1);
    println!("8. Is {} positive? {}", num1, is_positive);

    // Contoh 9
    let num2 = 56;
    let double = double_number(&num2);
    println!("9. Double of {} is {}", num2, double);

    // Contoh 10
    let num3 = 10;
    let is_even = check_even(&num3);
    println!("10. Is {} even? {}", num3, is_even);
}

fn check_even(n: &i32) -> bool {
    n % 2 == 0
}

fn double_number(n: &i32) -> i32 {
    *n * 2
}

fn check_positive(n: &i32) -> bool {
    *n > 0
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_number(n: &mut i32) -> i32{
    *n += 1;
    *n
}

fn print_number(n: &i32) -> i32{
    *n
}

fn change_vector(v: &mut Vec<i32>) -> Vec<i32>{
    v.push(4);
    v.clone()
}

fn print_vector(v: &Vec<i32>)  -> Vec<i32> {
    v.clone()

}

fn cal(s: &String) -> usize {
    s.len() // mengembalikan panjang string
}

fn get_first_char(s: &String) -> char {
    s.chars().next().unwrap()
}

fn main() {
    reference();
}
