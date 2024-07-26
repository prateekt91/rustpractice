use std::io;

fn main() {
   println!("Hello RS Cargo!");

   let mut input = String::new();

   io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    println!("You enetered: {}",input);

    let name: &str = "Pooja";
    if name == "Prateek" {
        println!("It works!!")
    } else {
        println!("Woila!!")
    }

}
