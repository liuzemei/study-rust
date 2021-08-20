const MAX_POINS: u32 = 10_0000;

fn main() {
    println!("Hello, world!");

    let x = 5;

    let x = x + 1;
    let x = x * 2;

    let x = true;
    let x = false;
    let x = 10 / 2; // i32
    let x = 49.0 / 11.0; // f64

    let y = "d"; // char

    let tup: (i32, f64, u8) = (500, 4.2, 1);

    // let (x, y, z) = tup;
    println!("{} {} {}", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]
}
