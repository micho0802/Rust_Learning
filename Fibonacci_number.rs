fn main() {
    println!("{}",fibonacci(9)) // Fibonacci number = 9
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