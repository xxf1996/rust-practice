// trait是共享行为体，有点类似于抽象类，专门用于让子类继承以实现细节；不过trait本身定义的方法可以有一个默认实现；
trait Vector {
  fn add(&self, another: &Self) -> Self;
  // 可以有一个默认实现。在实现时如果类型不实现这个方法的话就用默认实现
  fn say(&self) {
    println!("Vector类型打印");
  }
}

// T: trait xxx这种语法（trait bound）相当于指定泛型T为实现了某种trait的类型，impl trait xxx是这种语法的语法糖
#[derive(Debug)]
struct Vec2<T: Copy> {
  x: T,
  y: T,
}

#[derive(Debug)]
struct Vec3<T: Copy> {
  x: T,
  y: T,
  z: T,
}

// where语法可以为trait bound中的trait后置指定；通过`+`操作符可以合并trait（交集）
impl<T> Vector for Vec2<T> where T: Copy + std::ops::Add<Output = T> {
  fn add(&self, another: &Vec2<T>) -> Vec2<T> {
    let res = Vec2 {
      x: self.x + another.x,
      y: self.y + another.y,
    };
    res
  }
}

// std::ops::Add就是关于加法的trait
impl<T: Copy + std::fmt::Display + std::ops::Add<Output = T>> Vector for Vec3<T> {
  fn add(&self, another: &Vec3<T>) -> Vec3<T> {
    let res = Vec3 {
      x: self.x + another.x,
      y: self.y + another.y,
      z: self.z + another.z
    };
    res
  }
  // 也可以自定义实现
  fn say(&self) {
    println!("Vec3: ({}, {}, {})", self.x, self.y, self.z);
  }
}

fn vec_test() {
  let a = Vec2 {
    x: 1,
    y: 2
  };
  let b = Vec2 {
    x: -4,
    y: 5
  };
  let c = a.add(&b).add(&b);
  println!("a + b = {:?}", c);
  // let d = Vec2 {
  //   x: String::from("123"),
  //   y: String::from("33")
  // }; // 没有实现copy trait的类型当然不能使用
  c.say(); // 这里的方法就是默认实现
  let d = Vec3 {
    x: 12.3,
    y: -9.32,
    z: 323.02
  };
  d.say(); // 这里的方法就是自定义的
}

pub fn run() {
  vec_test();
}