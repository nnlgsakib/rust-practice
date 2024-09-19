// lesson 1
// Day 1-5: Rust Basics

// <-- STARTING  OF DAY ONE -->

use std::arch::x86_64::_mm256_div_ps;
use std::collections::HashMap;
use std::path::Prefix::DeviceNS;

// day1
// T2 : HELLO WORLD
fn T2() {
    println!("Hello, NLG!");
}

// T3 : VARIABLE
fn T3() {
    let x:i32 = 15;
    let mut y:f32 = 3.5;
    println!("value of x and y is {} . {}", x,y); // printing the values
    y = 100.2;// Modifing the y\
    println!("value of y after moding {}", y )
 }

//T4 :   Constants and Shadowing

fn T4() {
    const MAX_SCORE:i32 = 100;
    let score:i32   = 25;
    let score:i32 = score * 3;
    let score:i32 = score - 5 ;
    println!("the max score is : {} , and the current score is {}" , MAX_SCORE , score) // printing after shafowning
}

// <-- END OF DAY ONE -->

// <-- STARTING OF DAY 2 -->

// Day 2: Control Flow

// T1 : if statement

fn T1() {
    let mut age:i32 = 50;

    if age >= 18 {
        println!("you are an adult !")
    }
    else {println!("there's an error") }

    let status = "eligible";

    let age = if age >= 18 { status } else { "ineligible" };
    println!("{}",age)
}

// T2 : Loops in Rust
// infinite loop
fn loop_() {
    // printing 1-10 using rust loop
    let mut num = 0;
    loop {
        num += 1;
        println!("num {}", num);
        if num == 10 {
            break
        }
    }
}
//while loop
fn whilel() {
    let mut num = 2;
    while num <= 20 {
        println!("num is {}",num );
        num += 2;
    }
}

fn forloop(){
    let arr:[i32;5] =[1,2,3,4,5];
    for elem in arr {
        println!("the val is {:?}",elem)
    }
}

//  T3 : Match Expressions
fn MATCH() {
    let day = 1;
    match day {
        1 => println!("sat"),
        2 => println!("sun"),
        3 => println!("mon"),
        4 => println!("tues"),
        5 => println!("wed"),
        6 => println!("thurs"),
        7 => println!("fri"),
        _=> println!("invalid day!")

    }
}
// <-- END OF DAY 2 -->

// <-- STARTING OF DAY 3 -->

// Day 3: Ownership and Borrowing

//T1 : Ownership
fn own() {
    let massage = String::from("Ownership in Rust!");
    let new_massage = massage;
    println!("new massage is {}" , new_massage)

}
// T2 : Borrowing
fn borr() {
    let mut text = String::from("Rust borrowing"); // text is now mutable
    println!("the length is {:?}", lenx(&text)); // Immutable borrow for length check

    cng(&mut text); // Mutable borrow to change the string
    println!("after changing: {}", text); // Print the updated string
}

// Helper function to calculate length
fn lenx(s: &String) -> usize {
    s.len()
}

// Helper function to modify the string
fn cng(s: &mut String) {
    s.push_str(" is powerful");
}
// <-- END OF DAY 3 -->


// <-- STARTING OF DAY 4 -->

//Functions

//T1: Defining and Calling Functions

fn fnt() {
    println!("the product is {}",multiply(6,7))
}
fn multiply(x:i32,y:i32)->i32{
    x*y
}

//T2: Function Parameters and Return Types

fn FPRT() {
    println!("the sub  is {}",sub(15,8))
}

fn sub(x:i32,y:i32) -> i32{
    x-y
}

// T3: Expressions vs. Statements

fn EVS() {
    print!("the evaluation is {}",evaluate(100));
}

fn evaluate(x:i32) -> i32{
    x*2
}

// <-- END OF DAY 4 -->

// <-- STARTING OF DAY 5 -->

//Day 5 - Structs and Enums

// T1: Defining Structs

struct Book_1{
    Title:String,
    author:String,
    pages:i32,
    description: String,
    title: ()
}
fn strc() {
    let book= Book_1{
        Title:"The dark arts".to_string(),
        author: "THE NLG".to_string(),
        pages: 1000,
        description: "".to_string(),
        title: (),
    };
    println!("The book details are: \n The title: {} , \n The author: {} , \n Total pages : {}",book.Title,book.author,book.pages)
}

// T2: Method Syntax

// defining method
struct Book {
    title: String,
    author: String,
    pages: i32,
}

impl Book {
    // Method to describe the book
    fn description(&self) {
        println!(
            "The book '{}' is written by {} and has {} pages.",
            self.title, self.author, self.pages
        );
    }
}
fn imple_test() {
    let book = Book {
        title: "The Dark Arts".to_string(),
        author: "THE NLG".to_string(),
        pages: 1000,
    };

    // Calling the method to print book description
    book.description();
}

//T3: Defining Enums

// Def Status enume
enum Massge{
    Text(String),
    Image(String,i32),
    Video(String,u32)
}
fn print_massage(msg:Massge){
    match msg {
        Massge::Text(t) => println!("The title is {}",t),
        Massge::Image(u,s) => println!("the text url is {}  ,  and size in kb is {} KB", u,s),
        Massge::Video(u,d) => println!("The video url is '{}' and the lenght is {} : sec",u,d),
    }
}

fn enum_prc() {
    let text = Massge::Text(String::from("The art of numbers"));
    let thumb  = Massge::Image(String::from("https://youtube.com/muvid"),23);
    let vid = Massge::Video(String::from("https://youtube.com/muvid"),233);
    print_massage(text);
    print_massage(thumb);
    print_massage(vid)
}

// <-- END OF DAY 5 -->

// <-- STARTING OF DAY 6 -->

// Day 6 - Collections and Iterators

// T1: Vectors
fn vect() {
    let mut vec  = vec![2,3,4,3,3];

    println!("{}", vec.len());


    vec.push(33);
    println!("{:?}",&vec);

    vec.pop();
    println!(":{:?}",&vec);

    for vc in &vec{
        print!("{}",vc);
    }
}

//T 2: Strings

fn STR() {
    let mut st_r:String = String::from("Rust is fun ");
    st_r.push_str(" is fun");
    let ext:String = st_r+ &*"And I love to play with it ".to_string();
    println!("{}",ext);
    let slice = &ext[0..4];  // This will slice "Rust" from the string
    println!("Slice: {}", slice);

}

// T3: hashmap
fn hhashmap() {
    let mut studet_info = HashMap::new();
    // storing some values in student info
    studet_info.insert(String::from("sakib"),String::from("B"));
    studet_info.insert(String::from("mazid"),String::from("A-"));
    studet_info.insert(String::from("nasif"),String::from("A+"));

    // accesing a perticular value
    let student  =  String::from("sakib");
    match studet_info.get(&student) {
        Some(g) => println!("{}'s grade is {}",student, g),
        None => println!("studet not found"),
    }
    //itarating values using for loop
    for (k,v) in studet_info {
        println!("{}:{}",k,v)
    }
}
// <-- ENDING OF DAY  6 -->

// <-- STARTING OF DAY 7 -->

// Day 7: Error Handling

// T1: Using Result and Option

// using result
// Function to divide numbers and return a Result
fn divide_numbers(x: f32, y: f32) -> Result<f32, String> {
    if y == 0.0 {
        Err(String::from("There is an error / y cannot be 0"))
    } else {
        Ok(x / y)
    }
}

// Function to get user age using Option
fn get_user_age(a: Option<i32>) -> Option<i32> {
    match a {
        Some(age) => Some(age),
        None => None,
    }
}

fn main() {
    // Testing divide_numbers
    match divide_numbers(10.0, 0.0) {
        Ok(r) => println!("Result: {}", r),
        Err(e) => println!("Error: {}", e),
    }

    match divide_numbers(100.0, 343.0) {
        Ok(r) => println!("Result: {}", r),
        Err(e) => println!("{}", e),
    }

    // Testing get_user_age
    let age = Some(10);
    let no_age: Option<i32> = None;

    println!("User's age is {:?}", get_user_age(age));  // Some(10)
    println!("User's age is {:?}", get_user_age(no_age));  // None
}
// <-- ENDING OF DAY  7 -->

