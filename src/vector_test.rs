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



pub fn run() {
  create();
  read();
}
