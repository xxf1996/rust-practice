/**
 * Vec API: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
 **/

fn create() {
  // 创建vector时，元素为空必须指定元素类型
  let a: Vec<i32> = Vec::new();
  // a.push(1); // 不可变变量不能进行添加
  // vec!宏等同于Vec::new方法
  let b = vec!["name", "sser"];
  println!("{:?}", a);
  println!("{:?}", b);
  // 可变变量创建数组时可以为空，因为会自动根据添加的第一个元素来推断元素类型
  let mut c = Vec::new();
  c.push(32);
  // c.push("23"); // vector元素类型必须一致
  println!("{:?}", c);
}

fn read() {
  let a = vec![1, 3, 4, 2, 1];
  let b = a[0];
  let c = a.get(1); // get得到的居然是个Option？是出于下标可能越界的原因？

  println!("b: {}, c: {}", b, if let Some(v) = c {
    v
  } else {
    &(-1)
  });
  // let d = a[10]; // 索引方式访问越界会直接抛错；
}

fn loop_test() {
  let a = vec![3, 345, 4566, 234];
  for i in a { // 默认循环会直接移动元素，即循环完所有权就没了
    println!("{}", i);
  }
  // println!("{:?}", a); // 所以此处a的生命周期已经结束；
  let b = vec!["szdsf", "asfdd", "ef"];
  for i in &b { // 引用的方式循环就不会丢失所有权
    println!("{}", *i);
  }
  println!("{:?}", b); // 所以b可以继续使用，还有所有权
  // for i in &mut b {
    // 一样的，不可变变量无法进行可变引用
  // }
}

#[derive(Debug)]
enum RowType {
  Int(u32),
  Float(f64),
  String(String),
}

fn multi_type() {
  let a = vec![
    RowType::String(String::from("name")),
    RowType::Int(13),
    RowType::Float(203.23)
  ]; // 利用枚举可以实现vector元素为不同的类型
  println!("{:?}", a);
  for i in &a {
    match i {
      RowType::Float(val) => {
        println!("float element: {}", val);
      },
      RowType::Int(val) => {
        println!("int element: {}", val);
      },
      RowType::String(val) => {
        println!("string element: {}", val);
      }
    }
  }
  let b = &a[1]; // 如果直接使用enum元素，具体类型应该还需要match来匹配
  println!("单个元素：{:?}", b);
}

pub fn run() {
  create();
  read();
  loop_test();
  multi_type();
}
