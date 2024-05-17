// test
fn test1() { // define a function
    println!("Hello, world!");
} // D


fn test2() {
    let var: i32 = 100;
    let str: String = "Good".to_string(); // define string type
    println!("The value of var is: { }", var);
    println!("The value of str is: { }", str);
} // C


fn test3() {
    let (x, y) = (100, 200); // variable-binding
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
} // D


if ( test-expression) { // if true do this;
} else { // if false do this;
} // B


struct Name { // custom type
    member1: type,
    member2: type,
    ......
} // A


mod my_module { // define a module
    pub fn test(){ // pub means public attribute
    println!("Hello My Friends!");
    }
}

fn test6() {
    my_module::test(); // run the module
} // C


fn test7() {
    let v = vec! [100, 200, 300, 400]; // create a vector
    println!("First element is :{}",v[0]); // access the first element
    println!("Second element is :{}",v[1]);
    println!("Third element is :{}",v[2]);
    println!("Fourth element is :{}",v[3]);
} // D


impl Struct/Enum { // implement Struct or Enum
    fn method_name( &self ) -> type { // define a method
        self.member // access the member variable
    }
} // B


fn test9() {
    let var1: f32 = 100.88;
    let var2: i32 = var1 as i32; // convert data type
    println!("{}", var1);
    println!("{}", var2);
} // A


fn test10(){
    let x = "hello".to_string();
    let y = String::from("hello"); // assign a string value
    let z:&str = "hello";
    print!("{} {} {} ", x, y, z);
} // D


fn test11() {
    let mut num=5;
    loop { // loop statement
        println!("C# in {} Hours", num );
        if num == 8 {
            break; // break statement
        }
        num=num+1;
    }
} // C


enum Name { // custom type
    member1: type,
    member2: type,
    ......
} // A


mod ex_file; // loads an external file
use ex_file::ex_fun; // loads an external function

fn test13() {
    ex_fun(); // calls the external function
} // B


fn test14() {
    let num = 3;
    match num {
        1 => println!("one"),
        2 | 3 => println!("two or three"), // multiple patterns
        _ => println!("others"),
    }
} // D


impl Trait_Name for Struct/Enum{ // implement the trait
    fn trait_method(&self){ // implement the trait method
        self.member // access the member variable
    }
} // C


fn test16() {
    let num = funt(100); // calls the function
    println!("The value of num is: { }", num);
}

fn funt(num: i32) -> i32 { // specify a return type
    num + 200 // return a value to the caller
} // D


fn test17() {
    let a:[f32; 3] = [ 0.1, 0.2, 0.3 ]; // create array
    println!("{} {} {}",a[0],a[1],a[2]);
} // B


fn test18() {
    let t = ("R in", 8, "Hours", true); // create tuple
    print!("{} {} {} {}", t.0, t.1, t.2, t.3); // access the elements
} // A


fn test19() {
    let s=String::from("R in 8 Hours");
    let n=cal(&s); // reference
    println!("Value of the string is: {}",s);
    println!("Length of the string is: {}",n);
} 

fn cal(s:&String ) -> usize { // reference
    s.len() // get the length of the string
} // D


mod sup_module{ // parent module
    fn a() -> i32 {
        100
    }
    pub mod sub_module { // child module
        use super::a; // access parent function a
        pub fn b() {
            println!("{}",a()); // calls parent function a
        }
    }
}

fn test20() {
    sup_module::sub_module::b(); // call function b
} // C


fn test21() {
    let x = 5;
    match x {
        var @ 2 ..= 6 => println!("{}",var), // bind a variable to a range
        _ => println!("others"),
    }
} // D


fn test22() {
    let my_closure = | num: i32 | { num + 200 };
    // create a closure
    let num = 100;
    println!("{}", my_closure(num)); // call the closure
} // D


let x : Option<i32> = Some(100); // 'T' type is ?
let x : Option<bool> = Some(true); // 'T' type is ?
let x : Option<f32> = Some(100.5); // 'T' type is ?
let x : Option<char> = Some('A'); // 'T' type is ?
// B


fn test24() {
    let var : bool = false; // suppose it is false
    assert! (var == true); // check the error
    print!("{}", var);
} // C


fn main() { // define another function
    test1(); 
    test2();
    test3();
    test6();
    test7();
    test9();
    test10();
    test11();
    test14();
    test16();
    test17();
    test18();
    test19();
    test20();
    test21();
    test22();
    test24();
}