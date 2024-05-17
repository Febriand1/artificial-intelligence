// vector
fn example71() {
    let v =vec![100, 200, 300, 400]; // create a vector
    println!("First element is :{}",v[0]); // access the first element
    println!("Second element is :{}",v[1]);
    println!("Third element is :{}",v[2]);
    println!("Fourth element is :{}",v[3]);
}

fn example72() {
    let v =vec![8; 3]; // repeat three times
    println!("First element is :{}",v[0]);
    println!("Second element is :{}",v[1]);
    println!("Third element is :{}",v[2]);
}

fn example73() {
    let mut v=Vec::new(); // create a vector
    v.push('R'); // set R as the first element of vector
    v.push('U');
    v.push('B');
    v.push('Y');
    for n in v{
        print!("{} ",n);
    }
}

// Multiple Patterns
fn example74() {
    let num = 3;
    match num {
        1 => println!("\none"),
        2 | 3 => println!("\ntwo or three"), // multiple patterns
        _ => println!("\nothers"),
    }
}

// Range
// (... / ..=) is a range operator
fn example75() {
    let x = 3;
    match x {
        2 ..= 6 => println!("from 2 to 6"), // match from 2 to 6
        _ => println!("others"),
    }
}

// Binding a Range
fn example76() {
    let x = 5;
    match x {
        var @ 2 ..= 6 => println!("{}",var), // binding
        _ => println!("others"),
    }
}

// Generics
fn example77() {
    let x: Option<bool> = Some(true); // generic parameters
    let y: Option<i32> = Some(10);
    let z: Option<f64> = Some(20.88);
    let n: Option<i32> = None;
    match x {
        Some(x) => { println!("x = {}", x) },
        None => { println!("x = None") },
    }
    match y {
        Some(y) => { println!("y = {}", y) },
        None => { println!("y = None") },
    }
    match z {
        Some(z) => { println!("z = {}", z) },
        None => { println!("z = None") },
    }
    match n {
        Some(n) => { println!("n = {}", n) },
        None => { println!("n = None") },
    }
}

fn main() {
    example71();
    example72();
    example73();
    example74();
    example75();
    example76();
    example77();
}