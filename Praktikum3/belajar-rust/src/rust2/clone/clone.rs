// clone adalah cara untuk membuat salinan dari suatu nilai yang memiliki kepemilikan
// clone memungkinkan kita untuk memiliki dua variabel yang memiliki kepemilikan pada suatu waktu
fn clone() {
    // Contoh 1
    let s1 = String::from("clone");
    let s2 = s1.clone();
    println!("1. s1 = {}, s2 = {}", s1, s2);

    // Contoh 2
    let vec1 = vec![1, 2, 3];
    let vec2 = vec1.clone();
    println!("2. vec1 = {:?}, vec2 = {:?}", vec1, vec2);

    // Contoh 3
    let s3 = String::from("rust");
    let s4 = s3.clone();
    println!("3. s3 = {}, s4 = {}", s3, s4);

    // Contoh 4
    let s5 = String::from("ownership");
    let s6 = s5.clone();
    println!("4. s5 = {}, s6 = {}", s5, s6);

    // Contoh 5
    let vec3 = vec![4, 5, 6];
    let vec4 = vec3.clone();
    println!("5. vec3 = {:?}, vec4 = {:?}", vec3, vec4);

    // Contoh 6
    let s7 = String::from("clone");
    let s8 = s7.clone();
    println!("6. s7 = {}, s8 = {}", s7, s8);

    // Contoh 7
    let vec5 = vec![7, 8, 9];
    let vec6 = vec5.clone();
    println!("7. vec5 = {:?}, vec6 = {:?}", vec5, vec6);

    // Contoh 8
    let s9 = String::from("copy");
    let s10 = s9.clone();
    println!("8. s9 = {}, s10 = {}", s9, s10);

    // Contoh 9
    let vec7 = vec![10, 11, 12];
    let vec8 = vec7.clone();
    println!("9. vec7 = {:?}, vec8 = {:?}", vec7, vec8);

    // Contoh 10
    let s11 = String::from("clone example");
    let s12 = s11.clone();
    println!("10. s11 = {}, s12 = {}", s11, s12);
}

fn main() {
    clone();
}