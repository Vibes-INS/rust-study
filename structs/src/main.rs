struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn user_part() {
    let user1 = User {
        email: String::from("someone@emaple.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user: {}", user1.email);
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn rect_part() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle::new(10);
    let rect3 = Rectangle::new(50);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn main() {
    user_part();
    rect_part();
}
