fn main() {
    // if_1();
    // if_2();
    // loop_1();
    loop_2();
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


