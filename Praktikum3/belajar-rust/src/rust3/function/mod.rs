use crate::structs::*;

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

impl User {
    fn new(username: String) -> User {
        User { username, email: None }
    }

    fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }

    fn get_email(&self) -> Option<&String> {
        self.email.as_ref()
    }
}

impl Temperature {
    fn to_fahrenheit(&self) -> f64 {
        self.celsius * 1.8 + 32.0
    }
}

impl Email {
    fn new(address: String) -> Result<Email, String> {
        if address.contains('@') {
            Ok(Email { address })
        } else {
            Err(String::from("Invalid email address"))
        }
    }
}

impl Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * 3.14159 * self.radius
    }
}

impl Text {
    fn new(content: String) -> Text {
        Text { content }
    }

    fn to_uppercase(&self) -> String {
        self.content.to_uppercase()
    }
}

impl Numbers {
    fn new() -> Numbers {
        Numbers { values: vec![] }
    }

    fn add(&mut self, value: i32) {
        self.values.push(value);
    }

    fn sum(&self) -> i32 {
        self.values.iter().sum()
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

pub fn person() {
    let person = Person::new(String::from("Alice"), 30);
    person.greet();
}

pub fn rectangle() {
    let rectangle = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rectangle.area());
}

pub fn counter() {
    let mut counter = Counter { value: 0 };
    counter.increment();
    counter.increment();
    println!("The counter's value is {}.", counter.get_value());
}

pub fn user() {
    let mut user = User::new(String::from("john_doe"));
    user.set_email(String::from("john_doe@example.com"));
    println!("User email: {:?}", user.get_email());
}

pub fn temperature() {
    let temperature = Temperature { celsius: 22.5 };
    println!("The temperature in Fahrenheit is {}.", temperature.to_fahrenheit());
}

pub fn email() {
    match Email::new(String::from("user@example.com")) {
        Ok(email) => println!("Created email: {}", email.address),
        Err(err) => println!("Error: {}", err),
    }
}

pub fn circle() {
    let circle = Circle { radius: 2.5 };
    println!("The area of the circle is {} square pixels.", circle.area());
    println!("The circumference of the circle is {} pixels.", circle.circumference());
}

pub fn text() {
    let text = Text::new(String::from("hello"));
    println!("Text: {}", text.to_uppercase());
}

pub fn numbers() {
    let mut numbers = Numbers::new();
    numbers.add(5);
    numbers.add(10);
    println!("The sum of the numbers is {}.", numbers.sum());
}

pub fn point() {
    let point = Point::new(0, 0);
    println!("Is the point at the origin? {}", point.is_origin());
}