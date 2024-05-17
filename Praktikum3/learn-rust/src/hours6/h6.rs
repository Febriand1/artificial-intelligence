// module
mod my_module { // define a module
    pub fn test(){ // pub means public attribute
    println!("Hello My Friends!");
    }
}

fn example61(){
    my_module::test(); // run the module
}

// Embedded Module
mod m1 {
    pub fn a(){
    println!("m1 module");
    }

    pub mod m2{ // embedded module
        pub fn b(){
            println!("m2 module");
        }
    }
}

fn example62(){
    m1::a(); // run the module
    m1::m2::b(); // run the embedded module
}

// Private Function
// mod my_module1 {
//     pub fn a() { // function is public
//         println!("function a");
//     }
//     fn b(){ // function b is private
//         println!("function b");
//     }
// }

// fn example65() {
//     my_module1::a();
//     my_module1::b(); // call a private function
// }

mod my_module2 {
    pub fn a() {
        println!("function a");
        b(); // call a private function b
    }
    fn b() { // function b is private
        println! ("function b");
    }
}

fn example66() {
    my_module2::a();
}

// super
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

fn example67() {
    sup_module::sub_module::b(); // call function b
}

fn main() {
    example61();
    example62();
    // example65();
    example66();
    example67();
}