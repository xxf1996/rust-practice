struct Test {
    user: String,
    age: u8,
    name: Option<String>,
  }
  
  impl Test {
    fn say(&self) {
      println!("hello my name is {}", if self.name.is_some() { self.name });
    }
  }

fn borrow(str: &mut String) { // 可变引用，中途可以修改引用的值
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

fn main() {
    slice_test();
}

