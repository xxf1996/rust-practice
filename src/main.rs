mod mod_test; // 声明模块（相当于引入？好像不加这个就无法直接通过use来使用）
mod vector_test;
pub use crate::mod_test::inner_mod;

mod mod_test2 {
  pub fn just_see() -> u8 {
    234
  }
}

#[derive(Debug)]
struct Test {
  user: String,
  age: u8,
  name: Option<String>,
}

impl Test {
  fn say(&self) {
    println!("hello my name is {}", self.user);
  }
  fn test() {
    println!("这不就是静态方法么");
  }
}

#[derive(Debug)]
enum Ways {
  Left { dx: i16, dy: i16 },
  Right(Test),
  Mid
}

impl Ways {
  fn call(&self) -> &str {
    println!("Ways call");
    "some"
  }
}

fn borrow(str: &mut String) {
  // 可变引用，中途可以修改引用的值
  println!("{}", str);
  str.push_str("123");
}

// fn borrow2(str: &mut String) {
//     let s1 = &mut (*str);
//     let s2 = &(*str); // 同一作用域内，一个数据的可变引用和不可变引用（借用）不能同时存在！
//     println!("{}, {}", s1, s2);
// }

fn slice_test() {
  let mut test = String::from("Adssfsfdsf");
  let t1 = &mut test[1..3]; // 虽然String类型的slice可以设置成可变引用，但是引用结果为&str类型，str为字符字面量，无法修改；
  println!("{}", t1);
  // t1.clear();
}

fn struct_test() {
  let s1 = Test {
    user: String::from("sfdff"),
    age: 123,
    name: None,
  };
  s1.say();
  println!("{:?}", s1);
  println!("{:#?}", s1);
  Test::test();
}

fn match_test(x: Ways) -> String {
  match x {
    Ways::Mid => String::from("mid"),
    Ways::Left { dx, dy } => {
      println!("{}, {}", dx, dy);
      String::from("dx, dy")
    },
    Ways::Right(test) => {
      println!("{:?}", test);
      String::from(test.user)
    }
  }
}

fn match_test2(x: u8) {
  match x {
    0 => println!("0"),
    _ => println!("这不就是默认情况么？default？")
  }
}

fn match_test3(x: u8) {
  if let 0 = x {
    println!("这里就是匹配一种模式")
  } else {
    println!("else就等同于_匹配")
  }
}

fn option_test(x: Option<i8>) -> Option<i8> {
  if let Some(i) = x {
    Some(i + 1)
  } else {
    None
  }
}

fn enum_test() -> () {
  let a = Ways::Mid;
  println!("{:?}", a);
  println!("{}", a.call());
  // 匿名结构关联
  let b = Ways::Left { dx: 123, dy: 456 };
  println!("{:?}", b);
  // 函数关联？
  let c = Ways::Right(Test {
    user: String::from("sfdff"),
    age: 123,
    name: None,
  });
  println!("{:?}", c);
  println!("-----分割线-----");
  match_test(a);
  match_test(b);
  match_test(c);
  match_test2(0);
  match_test2(12);
  match_test3(0);
  match_test3(12);
}

fn main() {
  struct_test();
  enum_test();
  println!("{:?}", option_test(None));
  println!("{:?}", option_test(Some(8)));
  mod_test::you_say("he he");
  println!("{}", mod_test2::just_see());
  println!("{}", inner_mod::suibian());
  vector_test::run();
}
