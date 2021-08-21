// mod front_of_house {
//   pub mod hosting {
//     pub fn add_to_waitlist() {}
//     fn sear_at_table() {}
//   }

//   mod serving {
//     fn take_order() {}
//     fn serve_order() {}
//     fn take_payment() {}
//   }
// }

// 路径
// pub fn eat_at_restaurant() {
//   // 使用绝对路径来调用
//   crate::front_of_house::hosting::add_to_waitlist();

//   front_of_house::hosting::add_to_waitlist();
// }

// 访问父级模块

// fn serve_order() {}

// mod back_of_house {
//   fn fix_incorrect_order() {
//     cook_order();
//     // 相对路径来调用
//     super::serve_order();
//     // 绝对路径来调用
//     crate::serve_order()
//   }

//   fn cook_order() {}
// }

// 3. 路径访问实例

// mod three {

//   mod back_of_house {
//     pub struct Breakfast {
//       pub toast: String,
//       seasonal_fruit: String,
//     }
//     impl Breakfast {
//       pub fn summer(toast: &str) -> Breakfast {
//         Breakfast {
//           toast: String::from(toast),
//           seasonal_fruit: String::from("peaches"),
//         }
//       }
//     }
//   }

//   pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//     // meal.seasonal_fruit = String::from("blueberries"); // error 私有的没法访问
//   }
// }

// 4. use 关键字的测试
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

// use crate::front_of_house::hosting; // 使用绝对路径
// use front_of_house::hosting; // 使用相对路径
// pub fn eat_at_restaurant() {
//   hosting::add_to_waitlist();
//   hosting::add_to_waitlist();
//   hosting::add_to_waitlist();
//   hosting::add_to_waitlist();
// }

// use std::collections::HashMap;
// fn test() {
//   let mut map = HashMap::new();
//   map.insert(1, 2);
// }

// 5. as 关键字
// use std::fmt;
// use std::io;

// use std::fmt::Result as fmtResult;
// use std::io::Result as ioResult;

// fn f1() -> fmt::Result {}
// fn f2() -> io::Result {}

// fn f3() -> fmtResult {}
// fn f4() -> ioResult {}
