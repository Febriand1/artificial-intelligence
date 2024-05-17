// Error Checking
fn example91() {
    let check : bool = true; // suppose it is true
    assert!(check == true); // check the error
    println!("{}", check);
}

fn example92() {
    let check : bool = false; // suppose it is false
    assert!(check == true); // check the error
    println!("{}", check);
}    

fn main() {
    example91();
    example92();
}