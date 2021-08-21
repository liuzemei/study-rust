fn main() {
    println!("Hello, world!");
    let w = 30;
    let l = 50;
    println!("{}", area(w, l));
    println!("{}", area_tuple((w, l)));
    let rect = Rectangle {
        width: 30,
        length: 50,
    };
    println!("{}", area_struct(&rect));
    println!("{:?}", rect);
    println!("{:#?}", rect);

    println!("{:#?}", rect.area());

    let rect1 = Rectangle {
        width: 10,
        length: 30,
    };

    println!("{}", rect.can_hold(&rect1));

    let s = Rectangle::square(20);
    println!("{:#?}", s);
}

fn area(width: u32, length: u32) -> u32 {
    width * length
}
// 1. 用元组来接收
fn area_tuple(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

// 2. 用 struct 来接收
#[derive(Debug)] // 加上这个注解，就可以使用 {:?} 来打印了
struct Rectangle {
    width: u32,
    length: u32,
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}

// 3. 为 struct 添加方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}
