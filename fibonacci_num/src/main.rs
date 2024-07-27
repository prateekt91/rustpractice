use std::io;

fn main() {

    let mut input_str = String::new();
    println!("Welcome to Program for generating Fibonacci number!!");

    println!("Enter the nth num you want fibonacci for: ");
    io::stdin()
        .read_line(& mut input_str)
        .expect("Failed to Read!");

    let mut a = 0;
    let mut b = 1;
    let mut c;
    let input: u32 = input_str.trim()
                        .parse()
                        .expect("NOT a Number !!");
    
    if input == 0 {
        println!("Fibonnaci NUmber is: 0");
        return;
    }

    for _ in 1..=input {
        c = a + b;
        a = b;
        b = c;
    }

    println!("Fibonnaci NUmber is: {b}");

}
