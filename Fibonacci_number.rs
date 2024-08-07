use std::io;

fn main() {
    println!("Enter the Fibonacci number you want: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to read line");

    let n: u32 = input.trim().parse().expect("Please enter a valid number");
    
    println!("The Fibonacci number at position {} is {}", n, fibonacci(n)); 
}

fn fibonacci(n: u32) -> u32{
    let mut a: u32 = 0;
    let mut b: u32 = 1;

     if n == 0 {
        return a;
    } else if n == 1 {
        return b;
    } else {
        for _ in 2..=n {
            let c: u32 = a + b;
            a = b;
            b = c;
        }
        return b;
    }
}