fn main() {
    another_function(plus_one(5));
}

fn another_function(x: i32) {
    println!("the value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}
