use rand::Rng;
use std::cmp::Ordering;
use std::io; // prelude // trait 接口

fn main() {
    println!("猜数！");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字是： {}", secret_number);
    loop {
        println!("猜测一个数！");

        let mut guess = String::new(); // mut 是为了声明这是个可变的变量
        io::stdin().read_line(&mut guess).expect("无法读取行");
        println!("你猜测的数是： {}", guess);
        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // arm
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
