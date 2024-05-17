// Struct
struct Member { // create a struct
    id: i32, // member: type
    name: String,
    working: bool,
}

fn example51() {
    let clerk = Member { // initialize the struct
        id: 016320, // member: value
        name: "Smith".to_string(),
        working: true,
    };

    println!("ID is {}", clerk.id); // access the members
    println!("Name is {}", clerk.name);
    println!("Working is {}", clerk.working);
}

struct Square { // create a struct
    len: i32,
    wid: i32, 
}

fn example52() {
    let table = Square { len: 10, wid: 8 }; // initialization
    println!("The area is {}", table.len * table.wid); // access
}

// Enum
enum Language{ // define an enum
    JS, // member
    GO,
    VB,
}

fn program(var: Language) {
    match var{ // using match statement
        Language::JS => println!("JS in 8 Hours"),
        Language::GO => println!("GO in 8 Hours"),
        Language::VB => println!("VB in 8 Hours"),
    }
}

fn example53() {
    program(Language::JS); // call the function
    program(Language::GO);
    program(Language::VB);
}

// Ownership
fn example54() {
    let x = String::from("try"); // x owns “try”
    let y = x; // Warning! The ownership of x moves to y
    // println!("{}", x); // Error! x is no longer available
    println!("{}", y); // y owns “try”
}

// fn example55() {
//     let s = String::from("R in 8 Hours"); // s owns “R in 8 Hours”
//     let n = cal(s); // Warning! s will lose the ownership after used
//     println!("Value of the string is: {}",s); // s is no longer available
//     println!("Length of the string is: {}",n);
// }

// fn cal(s:String) -> usize {
//     s.len() // get the length of the string
// }

// Reference
fn example56() {
    let s = String::from("R in 8 Hours"); // s owns “R in 8 Hours”
    let n = cal(&s); // Warning! s will lose the ownership after used
    println!("Value of the string is: {}",s); // s is no longer available
    println!("Length of the string is: {}",n);
}

fn cal(s:&String) -> usize {
    s.len() // get the length of the string
}

fn main() {
    example51();
    example52();
    example53();
    example54();
    // example55();
    example56();
}
    