// method
struct Circle{ // create a struct type
    radius: f32, // struct member
}

impl Circle{ // implement the struct
    fn area(&self) -> f32{ // define a method
        std::f32::consts::PI * self.radius * self.radius
    } // method body
}

fn example81() {
    let obj = Circle { radius : 2000.00}; // create a struct object
    println!("The Circle area is: {}", obj.area()); // call the method
}

// trait
trait Calculate{ // define a trait
    fn area(&self) -> f32; // define a trait method
}

impl Calculate for Circle{ // implement the trait
    fn area(&self) -> f32{ // implement the trait method
        std::f32::consts::PI * self.radius * self.radius
    }
}

fn example82() {
    let obj = Circle { radius : 2000.00}; // create a struct object
    println!("The Circle area is: {}", obj.area()); // call the method
}

pub trait Show { // define a trait
    fn show(&self); // define a trait method
}

impl<T> Show for T // implement the trait with generic

where T: ToString{ // specify the String type
    fn show (&self){ // implement the trait method
        println!("{}",self.to_string());
    }
}

fn example83() {
    String::from("C# in 8 Hours").show(); // call method
}

// Drop() Method
struct Game {
    number: i32,
}

impl Drop for Game {
    fn drop(&mut self) { // define a drop method
        println!("The #{ } Winner.", self.number);
    }
}

fn example84() {
    let _baseball = Game { number: 3 };
    let _football = Game { number: 2 };
    let _basketball = Game { number: 1 };
} 

// Closure
fn example85() {
    let my_closure = | num: i32 | { num + 200 }; // create a closure
    let num = 100;
    println!("{}", my_closure(num)); // call the closure
}

fn example86() {
    let mut capacity = "Hard disk capacity: 5000".to_string();
    {
        let mut my_closure = | c: char |{capacity.push(c)}; // closure
        my_closure('G'); // call the closure
    }
    println!("{:?}", capacity); // {:?} is used to output a string
}

fn main() {
    example81();
    example82();
    example83();
    example84();
    example85();
    example86();
}