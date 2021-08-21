use std::collections::HashMap;
fn main() {
    vector_1();
    vector_2();
}

fn vector_1() {
    // 1. 创建 Vector
    // let v: Vec<i32> = Vec::new();
    // 2. 使用初始值创建 Vector
    // let v = vec![1, 2, 3];

    // 3. 先new，再push，就可以自动类型推断了。
    let mut v = Vec::new();
    v.push(1);
    v.push(3);
    v.push(2);

    // 4. 删除 作用域过了，就都被删除了，元素也都被删了

    // 5. 获取
    // 使用 索引的话，程序会 paic
    // 使用 get 的话，不会
    // println!("The third element is {}", v[100]);
    // println!("The third element is {}", v.get(2));
    match v.get(100) {
        Some(third) => println!("the third element is {}", third),
        None => println!("There is no third element"),
    }

    // 6. Vector 的遍历
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn vector_2() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(0.3),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}

fn string_1() {
    // 1. 创建字符串
    let s1 = String::new();
    let s2 = "haha".to_string();
    let s3 = String::from("haha");

    // 2. 更新 String
    let mut s4 = String::from("foo");
    s4.push_str("bar");
    println!("{}", s4);

    // 3. 字符串的拼接
    let s5 = s2 + &s3; // 注意这里，s2以后就不能用了，s3可以

    let s = format!("{}-{}-{}", s1, s4, s3); // 这个不会影响变量的可用性。
}

fn hashmap_1() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 50);
    scores.insert(String::from("red"), 100);
}

fn hashmap_2() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();
}
