use std::collections::HashMap;

fn create_test() {
  // let a = HashMap::new(); // 不变的值压根没法创建hashmap
  // a.insert("123", 123);
  let mut a = HashMap::new();
  a.insert(String::from("name"), 123); // k和v的类型会从第一个插入的键值对自动推断
  // a.insert("other", "xxx"); // k和v的类型必须保持不变
  let key = String::from("key");
  a.insert(key, 1024); // 同所有的函数传参一样，如果参数不是可copy的且不是引用，那么所有权就会转移到函数内，即move
  // println!("{}", key); // 所以这里key变量已经没有所有权了
  let val = 2048;
  a.insert(String::from("val"), val);
  println!("{}", val); // 因为i32这种值可以被copy，所以所有权不会转移
}

fn use_test() {
  let mut b = HashMap::new();
  b.insert(String::from("name"), String::from("wtf"));
  let key = String::from("name");
  let val = b.get(&key); // get访问得到是一个Option<&V>；
  println!("b の name：{}", if let Some(v) = val {
    v
  } else {
    "空的"
  });
  let key2 = String::from("other");
  let val2 = b.get(&key2);
  println!("b の other：{}", if let Some(v) = val2 {
    v
  } else {
    "空的"
  });
}

fn loop_test() {
  let mut a = HashMap::new();
  a.insert(String::from("name"), String::from("xxx"));
  a.insert(String::from("some"), String::from("yyy"));
  a.insert(String::from("other"), String::from("zzz"));

  for (key, val) in &a {
    println!("key: {}, val: {}", key, val);
  }

  println!("{:?}", a); // 使用引用的话就保留了所有权

  for (key, val) in a {
    println!("key: {}, val: {}", key, val);
  }
  // println!("{:?}", a); // 同理，这样迭代a的所有权已经被转移了
}

fn update_test() {
  let mut a = HashMap::new();
  a.insert(String::from("name"), String::from("xxx"));
  a.insert(String::from("name"), String::from("xxx2")); // 使用insert可以直接覆盖
  let name = String::from("name");
  println!("name is {}", if let Some(val) = a.get(&name) {
    val
  } else {
    "空的"
  });
  a.entry(String::from("name")).or_insert(String::from("xxx3")); // entry的or_insert是先判断之前该键是否有值，如有值则不覆盖，而无值则覆盖
  println!("name is {}", if let Some(val) = a.get(&name) {
    val
  } else {
    "空的"
  });
  a.entry(String::from("other")).or_insert(String::from("xxx3"));
  let other = String::from("other");
  println!("other is {}", if let Some(val) = a.get(&other) {
    val
  } else {
    "空的"
  });
  let b = a.entry(String::from("other")).or_insert(String::from("xxx4")); // or_insert方法会返回对应键的可变引用
  *b = String::from("changed");
  println!("other is {}", if let Some(val) = a.get(&other) {
    val
  } else {
    "空的"
  });
}

pub fn run() {
  create_test();
  use_test();
  loop_test();
  update_test();
}
