struct Test {
  user: String;
  age: u8;
  name: Option<String>;
}

impl Test {
  say(&self) {
    println!("hello my name is {}", self.name);
  }
}

