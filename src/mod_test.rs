pub fn you_say(some: &str) {
  println!("you say:{}", some)
}

pub mod inner_mod {
  // 即使模块变成公共的，内部方法没有显式声明为公共方法也无法被外部使用！
  fn some_sth() -> u8 {
    111
  }
  pub fn suibian() -> String {
    String::from("suibian")
  }
}

// mod默认私有！
mod inner_mod2 {
  pub fn no_name() -> u8 {
    234
  }
}
