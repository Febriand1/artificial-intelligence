// If Statement
fn example41(){
    let num = 10;
    if num == 10{
        println!("num is equal to 10");
    }
}

// If-else Statement
fn example42(){
    let x = 100;
    let y = 200;
    if x > y{
        println!("x is greater than y");
    } else {
        println!("x is smaller than y");
    }
}

// Let-If Statement
fn example43(){
    let num = if true{ // let-if statement
        100
    } else {
        200
    };
    println!("The value of num is {}", num)
}

// Loop – Break Statement
fn example44(){
    let mut num = 5;
    loop{ // loop statement
        println!("C# in {} Hours", num );
        if num == 8 {
            break; // break statement
        }
        num = num+1;
    }
}

// For Statement
fn example45(){
    for num in 5..9 { // “5..9” contains numbers from 5 to 8
        println!("Java in {} Hours", num);
    }
}

// While Statement
fn example46(){
    let mut num = 0;
    while num <= 8 { // while statement
        print!("{} ", num);
        num = num+1;
    }
}

// Tuples
fn example47(){
    let t = ("Python in", 8, "Hours", true); // create a tuple
    println!("\n{} {} {} {}", t.0, t.1, t.2, t.3); // access the elements
}

// Match
fn example48(){
    let num:i32 = 3; // given expression
    match num {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"), // match this
        4 => println!("four"),
        _ => println!("something else"),
    }
}

fn main() {
    example41();
    example42();
    example43();
    example44();
    example45();
    example46();
    example47();
    example48();
}