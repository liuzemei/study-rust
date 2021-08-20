fn main() {
    println!("Hello, world!");
    println!("{}", five());
    println!("plus five {}", plus_five(6));
}

fn five() -> i32 {
    5
}

fn plus_five(x: i32) -> i32 {
    x + 5
}
