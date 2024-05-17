// variables
fn example21() {
    let var = "OK";
    println!("The value of var is: {}", var);
}

fn example22() {
    let var: i32 = 100;
    let str: String = "Good".to_string();
    println!("The value of var is: {}", var);
    println!("The value of str is: {}", str);
}

// output format
fn example23() {
    let x = 100;
    let y = 200;
    let z = 300;
    println!("{ }", x);
    println!("{ } { }", y, z);
}

// constants
const NUM: i32 = 100;

fn example24() {
    println!("The value of NUM is {}", NUM);
}

// Date Type Conversion
fn example25() {
    let var1: f32 = 100.88;
    let var2: i32 = var1 as i32;
    println!("{}", var1);
    println!("{}", var2);
}

// function
fn funt(x:i32, y:i32) { // function
    println!("The Sum is: {}", x + y);
}

fn example26() {
    funt(100, 200);
}

// return type
fn funt1(num: i32) -> i32 { // specify a return type
    num + 200 // return a value to the caller
}

fn example27() {
    let num = funt1(100); // calls the function
    println!("The value of num is: {}", num);
}

fn foo() -> bool { // specify a return type
    return true // return a value to the caller
}

fn example28() {
    let b = foo(); // foo() is a caller
    println!("The result is: {}", b);
}

fn main() { // main function
    example21();
    example22();
    example23();
    example24();
    example25();
    example26();
    example27();
    example28();
}
