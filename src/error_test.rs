use std::fs::File;
use std::io::Read;
use std::io::Error;

// RUST_BACKTRACE=1 cargo run  可以显示panic!发生时的调用栈
fn some_err() {
  panic!("some error");
}

fn result_test() {
  let f = File::open("not-found.txt"); // 简言之，Result就是一个拥有两种状态的枚举类型，因为其结果进行使用时要用match匹配
  match f {
    Ok(file) => {
      println!("open not-found.txt success: {:?}", file)
    },
    Err(err) => {
      println!("open not-found.txt error: {:?}", err)
    },
  }
}

// 通过直接返回函数体内对应的Err，从而达到Result的向上传播，有点类似于throw？
fn error_up() -> Result<String, Error> {
  let mut res = String::from("");
  // ?符号可以用在返回Result类型的方法后，当匹配到结果为Err类型时，自动返回对应的Err类型并中止后续逻辑
  // 反之如果匹配到为Ok类型时，则继续执行后续代码，且支持链式调用（即对Ok(Type)中的Type类型进行后续操作）；
  File::open("not-found.txt")?.read_to_string(& mut res)?;
  Ok(res)
}

// fn error_up2() {
//   let mut res = String::from("");
//   // ?符号只能在返回Result类型的函数中使用
//   File::open("not-found.txt")?.read_to_string(& mut res)?;
//   Ok(res)
// }

fn unwrap_test() {
  // unwrap是一种处理Result的快捷match；Ok类型返回里面的值，Err类型则直接panic
  File::open("not-found.txt").unwrap();
}

fn expect_test() {
  // expect是另一种快捷match；Ok类型返回里面的值，Err类型则直接panic且错误消息为参数内容
  File::open("not-found.txt").expect("没错，这里可以自定义Error消息");
}

pub fn run() {
  result_test();
  let test1 = error_up();
  match test1 {
    Ok(res) => println!("error_up ok: {}", res),
    Err(err) => println!("error_up error: {:?}", err)
  }
  // some_err();
  // unwrap_test();
  expect_test();
}
