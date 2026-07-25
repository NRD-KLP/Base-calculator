use std::io;
fn main() {
    println!("---CALCULATOR---");
    loop {
        println!("Write here your first number");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let a: u32 = input.trim().parse().unwrap();
        println!("Write here your second number");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).unwrap();
        let b: u32 = input1.trim().parse().unwrap();
        println!("chose arithmetic operation ('+', '-', '*', '/',  '%', '^')");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).unwrap();
        let operation = operation.trim(); 
            match operation {
            "+" => println!("Answer: {}", a + b),
            "-" => println!("Answer: {}", a - b),
            "*" => println!("Answer: {}", a * b),
            "%" => println!("Answer: {}", a % b),
            "^" => println!("Answer: {}", a.pow(b)),
            "/" => {
                if b == 0 {
                    println!("Error!");
                } else {
                    println!("Answer: {}", a / b);
                }
            }
            _ => println!("Unknown operation.")
        }
        println!("Do you want to leave? (yes/__)");
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        if ans.trim() == "yes" {
            break;
        } 
    }
    println!("Operation is over. Thank you for using!")
}