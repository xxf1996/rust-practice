fn inner_return(name: &str) -> String {
  let res = format!("内部返回引用: {}", name);
  // res.as_str() // 内部的变量无法返回引用
  res // 内部变量只能通过move（转移所有权）返回
}

// 只有在要返回输入数据中的引用时才需要指定生命周期参数
// 生命周期参数本质上是一种泛型，以'开头
fn select_one<'a>(a: &'a str, b: &'a str) -> &'a str {
  if a.contains("s") {
    a
  } else {
    b
  }
}

pub fn run() {
  let str1 = "demm1";
  let str2 = inner_return(str1);
  println!("str2: {}", str2);
  let str3 = "sb";
  let str4 = select_one(str3, str1);
  println!("str4: {}", str4);
  let str6;
  // {
  //   let str5 = String::from("分身大师都是跟");
  //   str6 = select_one(str1, str5.as_str());
  //   // str5的生命周期到这就结束了
  // }
  // println!("str6: {}", str6);
  {
    let str5 = "分身大师都是跟"; // 等同于&'static str
    str6 = select_one(str1, str5);
    // str5的生命周期是静态生命周期，即全局有效；但是所有权依然没了
  }
  println!("str6: {}", str6);
}
