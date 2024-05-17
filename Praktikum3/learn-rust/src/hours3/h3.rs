// variable-binding
fn example31(){
    let (x, y) = (100, 200); // variable-binding
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn example32(){
    let mut a = 100;
    let mut b = 200;
    a = a + 300;
    b = b + 400;
    println!("Finally a is {}", a);
    println!("Finally b is {}", b);
}

// String Assignment
fn example33(){
    let x = "hello".to_string();
    let y = String::from("hello");
    let z:&str = "hello";
    println!("{} {} {} ", x, y, z);
}

// Arithmetical Operators
fn example34(){
    println!("10 + 2 = {}", 10 + 2);
    println!("10 - 2 = {}", 10 - 2);
    println!("10 * 2 = {}", 10 * 2);
    println!("10 / 2 = {}", 10 / 2);
    println!("10 % 2 = {}", 10 % 2);
}

// Logical Operators
fn example35(){
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
}

// Comparison Operators
fn example36(){
    let x:i32 = 100;
    let y:i32 = 200;
    println!("x is greater than y : {}", x > y);
    println!("x is less than y : {}", x < y);
    println!("x is unequal to y : {}", x != y);
    println!("x is greater/equal to y : {}", x >= y);
    println!("x is less/equal to y : {}", x <= y);
    println!("x is completely equal to y : {}", x == y);
}

// Array
fn example37(){
    let mut a: [i32; 4] = [8; 4]; // create an array
    a[1] = 10;
    a[2] = 20;
    println!("array i32: {} {} {} {}",a[0], a[1], a[2], a[3]);
}

fn example38(){
    let a:[f32; 4] = [0.1, 0.2, 0.3, 0.4]; // create an array
    println!("array f32: {} {} {} {}",a[0],a[1],a[2],a[3])
}

// Slice
fn example39(){
    let a = [0, 10, 20, 30, 40, 50, 60]; // create an array
    let slice = &a[2..5]; // extract from a[2] to a[4]
    println!("slice 0: {}",slice[0] ); // show slice’s elements
    println!("slice 1: {}",slice[1] );
    println!("slice 2: {}",slice[2] );
}

fn main(){
    example31();
    example32();
    example33();
    example34();
    example35();
    example36();
    example37();
    example38();
    example39();
}