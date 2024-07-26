use std::io;


fn main() {
    println!("Hello, Welcome to Temp Converter!");
    println!("Lets Convert Fahrenheit to Celcius!!");
    let mut temp_str = String::new();

    println!("Enter temp in Fahrenheit:");
    io::stdin()
        .read_line(&mut temp_str)
        .expect("Failed to Read Tempurature");
    
    let temp: i32 = temp_str.trim()
                            .parse()
                            .expect("Input value is not an integer!");

    let con_temp = (temp - 32) * 5/9;
    println!("Tempurature in Celcius: {}", {con_temp});
}
