

//Entry point of the program
// #![allow(warnings)]

use std::collections::HashMap;
fn main() {
    
    hello_world();
    types();
    learn_functions();
    ownership();
    borrowing();
    mutable_immutable();
    mutablity();
    constants();
    shadowing();
    comments();
    control_flow();
    looping();
    structs();
    enums();
    error_handling();
    collection_types();
    utf_encoded_strings();
    hash_maps();

    async_();
}

fn async_() {
    
}

fn hash_maps(){
    let mut scores = HashMap::new();
    let blue = "Blue";
    let team_name_blue = String::from(blue);
    scores.insert(team_name_blue, 10);
    scores.insert(String::from("Yellow"), 50);

    
    let score_blue = scores.get(&String::from(blue)).copied().unwrap_or(0);
    println!("Score of Blue team: {}", score_blue);

    for(key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn utf_encoded_strings(){
    let s1 = "whathever".to_string();
    let s2 = String::from("Whatever");
    let mut s = String::from("hello");
    s.push_str("bar");
    s.push('!');
    println!("{}", s);
    let s3 = s1 + &s2;
    println!("{}", s3);
}

fn collection_types() {
    let _v:Vec<i32> = Vec::new();
    let mut the_vec = vec![1,2,3];
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    the_vec.push(5);
    the_vec.push(6);
    the_vec.push(7);
    println!("{:?}", v);
    println!("{:?}", the_vec);

    // Direct indexing
    let second: &i32 = &the_vec[1];
    println!("The second element is: {second}"); 

    let third = the_vec.get(2);
    match third {
        Some(third) => println!("The third element is: {third}"),
        None => println!("There is no third element")
    }
}

fn error_handling(){
    // enum Option<T>{
    //     Some(T),
    //     None
    // }
    // enum Result<T, E>{
    //     Ok(T),
    //     Error(E)
    // }
    fn divide_using_option(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }
    let result = divide_using_option(10.0, 2.0);
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero")
    }
    fn divide_using_result(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(numerator / denominator)
        }
    }
    let result = divide_using_result(10.0, 5.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(message) => println!("{}", message)
    }
}

fn enums() {
    enum IpAddrKind {
        V4,
        V6
    }
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    fn route(ip_kind: IpAddrKind) {
        match ip_kind {
            IpAddrKind::V4 => println!("V4"),
            IpAddrKind::V6 => println!("V6")
        }
    }
    route(four);
    route(six);
    //Using structs
    struct IpAddr {
        kind: IpAddrKind,
        address: String
    }
    let _home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let _loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    enum IpAddrEnum {
        V4(String),
        V6(String)
    }
    let _home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrEnum::V6(String::from("::1"));

    enum IpAddrEnum2 {
        V4(u8, u8, u8, u8),
        V6(String)
    }
    let _home = IpAddrEnum2::V4(127, 0, 0, 1);
    let _loopback = IpAddrEnum2::V6(String::from("::1"));

}

fn structs(){
    // tuple
    let _rect = (200,500);

    // struct
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool
    }

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    }

    let mut user1 = User{
        username: String::from("John"),
        email: String::from("john@john.com"),
        sign_in_count: 1,
        active: true
    };

    user1.email = String::from("john@gmail.com");
    println!("User1 email: {}", user1.email);

    fn build_user(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true
        }
    }
    let _user2 = User{
        username: String::from("Jane"),
        ..user1
    };

    //Tuple structs
    struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _white = Color(255, 255, 255);

    // Unit-like struct
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}

fn looping(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }
    let b = ["apple", "banana", "cherry"];
    for element in b {
        println!("The value is: {}", element);
    }
}

fn control_flow(){
    let number = 3;
    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }
    let number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn comments(){
    // This is a line comment
    println!("Hello, Rust world!");
    /*
    This is a block comment
    */
    println!("Hello, Rust world!");
    let _x = 5;
    let _y = 6;
}

fn shadowing(){
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("x: {}", x);
    
    }
    println!("x: {}", x);
    let _spaces = "   ";
    let _spaces = _spaces.len();
}

const SOME_CONSTANT: u32 = 10_000;

fn constants(){
    let x = 5;
    const MAX_POINTS: u32 = 100_000;
    println!("x: {}", x);
    println!("MAX_POINTS: {}", MAX_POINTS);
    println!("SOME_CONSTANT: {}", SOME_CONSTANT);
}

fn mutablity(){
    let mut a = 5;
    println!("a: {}", a);
    a = 10;
    println!("a: {}", a);
}

struct BankAccount {
    owner: String,
    balance: f64
}
impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account of {}", amount, self.owner);
        if self.balance >= amount {
            self.balance -= amount;
        } else {
            println!("Insufficient funds");
        }
    }
    fn check_balance(&self){
        println!("Balance of account of {}: {}", self.owner, self.balance);
    
    }
}

fn mutable_immutable(){
    let mut account = BankAccount{
        owner: "John".to_string(),
        balance: 100.0
    };
    account.check_balance();
    account.withdraw(50.0);
    account.check_balance();
}

fn borrowing(){
    let mut x=5;
    let r=&mut x;
    *r += 1;
    *r -= 3;
    // only one mutable reference or many immutable
    // println!("x: {}", x);
    println!("r: {}", r);
}

fn ownership(){
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("len of {}: {}", s1, len);
    let s2 = s1;
    println!("s1: {}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn learn_functions() {
    let height: u32 = 180;
    tell_height(height);
    human_id("John", 30, 180.5, true);

    let _x: i32 = {
        let price: i32 = 100;
        let quantity: i32 = 10;
        price * quantity
    };
    println!("The value of _x is: {}", _x);
    add(4, 6);
    let y = add(4, 6);
    println!("The value of y is: {}", y);
    let bmi = calculate_bmi(70.0, 1.75);
    println!("The value of bmi is: {}", bmi);
}
//BMI
fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn types(){
    println!("Hello, Rust test cargo!");
    //int, float, bool, char
    let x: i32 = -42;
    let y: u64 = 100;
    println!("signed x: {}, unsigned y: {}", x, y);
    let z: f64 = 3.14;
    println!("float z: {}", z);
    let t: bool = true;
    println!("bool t: {}", t);
    let c: char = 'A';
    println!("char c: {}", c);
    //arrays, tuples, slices, and strings(slice string)
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array numbers: {:?}", numbers);
    // let mix = [1,2,"apple", true];
    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("array fruits: {:?}", fruits);
    println!("array fruits: {}", fruits[0]);
    let human: (String, i32, bool) = ("John".to_string(), 30, true);
    println!("tuple human: {:?}", human);
    let mix_tuple = ("John", 30, true, [1, 2, 3]);
    println!("tuple mix_tuple: {:?}", mix_tuple);
    //slices: a reference to a contiguous sequence of elements in a collection
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("slice number_slices: {:?}", number_slices);
    let fruit_slices: &[&str] = &["apple", "banana", "cherry"];
    println!("slice fruit_slices: {:?}", fruit_slices);
    //string slice
    let book_slices: &[&String] = &[&"Rust".to_string(), &"Python".to_string(), &"Java".to_string()];
    println!("slice book_slices: {:?}", book_slices);
    // Strings Vs String slices
    let mut book: String = String::from("Rust");
    book.push_str(" Programming");
    println!("String book: {}", book);
    let string_heap: String = String::from("Rust Programming");
    let slice: &str = &string_heap[0..4];
    println!("String slice: {}", slice);
    println!("Hello, rust world!");
}

fn tell_height(height: u32){
    println!("Height is: {}", height);
}

fn human_id(name: &str, age: u32, height: f32, is_alive: bool) {
    println!("Name: {}, Age: {}, Height: {}, Alive: {}", name, age, height, is_alive);
}

fn hello_world() {
    println!("Hello, world!");
}





