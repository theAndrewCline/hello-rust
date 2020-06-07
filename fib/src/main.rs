fn fib(x: u32) -> u32 {
    return match x {
        0 => 0,
        1 => 1,
        _ => fib(x - 2) + fib(x - 1),
    };
}

fn main() {
    println!("fib of 0 = {}, 0", fib(0));
    println!("fib of 1 = {}, 1", fib(1));
    println!("fib of 2 = {}, 1", fib(2));
    println!("fib of 3 = {}, 2", fib(3));
    println!("fib of 4 = {}, 3", fib(4));
    println!("fib of 5 = {}, 5", fib(5));
    println!("fib of 25 = {}, 75,025", fib(25));
}
