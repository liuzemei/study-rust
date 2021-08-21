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

fn five() -> i32 {
    5
}

fn plus_five(x: i32) -> i32 {
    x + 5
}

fn if_1() {
    let number = 6;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_2() {
    let number = 6;

    let t = if number < 5 {
        println!("condition was true");
        5
    } else {
        println!("condition was false");
        6
    };
    println!("The value of t is: {}", t);
}

fn loop_1() {
    loop {
        println!("again!"); // 无限循环
    }
}

fn loop_2() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_1() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_1() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn for_2 (){
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}


