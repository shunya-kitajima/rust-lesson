struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

struct Rectangle {
  width: u32,
  height: u32,
}
impl Rectangle {
  fn create(width: u32, height: u32) -> Self {
    Self { width, height }
  }
  fn area(&self) {
    println!("{}", self.width * self.height);
  }
}

pub fn run() {
  let user1 = User {
    username: String::from("someusername123"),
    email: String::from("someone@example123"),
    sign_in_count: 1,
    active: true,
  };
  let mut user1 = User {
    username: String::from("someusername123"),
    email: String::from("someone@example123"),
    sign_in_count: 1,
    active: true,
  };
  user1.email = String::from("anotheremail@example.com");
  println!("{:#?}", user1);

  let user2 = build_user(String::from("user2"), String::from("user2@xxx.com"));
  println!("{:#?}", user2);

  let rect = Rectangle::create(30, 30);
  println!("{:#?}", rect);
  rect.area();
  println!("{:#?}", rect);
}

fn build_user(username: String, email: String) -> User {
  User {
    username: username,
    email: email,
    sign_in_count: 1,
    active: true,
  }
}
